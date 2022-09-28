/**
 * This module converts a package containing D Model definitions to
 * $(REF SerializedModels, dorm,declarative)
 */
module dorm.declarative.conversion;

import dorm.annotations;
import dorm.declarative;
import dorm.model;

import std.datetime;
import std.meta;
import std.traits;
import std.typecons;

version (unittest) import dorm.model;

/** 
 * Entry point to the Model (class) to SerializedModels (declarative) conversion
 * code. Manually calling this should not be neccessary as the
 * $(REF RegisterModels, dorm,declarative,entrypoint) mixin will call this instead.
 */
SerializedModels processModelsToDeclarations(alias mod)()
{
	SerializedModels ret;

	static foreach (member; __traits(allMembers, mod))
	{
		static if (__traits(compiles, is(__traits(getMember, mod, member) : Model))
			&& is(__traits(getMember, mod, member) : Model)
			&& !__traits(isAbstractClass, __traits(getMember, mod, member)))
		{
			processModel!(
				__traits(getMember, mod, member),
				SourceLocation(__traits(getLocation, __traits(getMember, mod, member)))
			)(ret);
		}
	}

	return ret;
}

private void processModel(TModel : Model, SourceLocation loc)(
	ref SerializedModels models)
{
	ModelFormat serialized = DormLayout!TModel;
	serialized.definedAt = loc;
	models.models ~= serialized;
}

enum DormLayout(TModel : Model) = delegate() {
	ModelFormat serialized;
	serialized.tableName = TModel.stringof.toSnakeCase;

	alias attributes = __traits(getAttributes, TModel);

	static foreach (attribute; attributes)
	{
		static if (isDBAttribute!attribute)
		{
			static assert(false, "Unsupported attribute " ~ attribute.stringof ~ " on " ~ TModel.stringof ~ "." ~ member);
		}
	}

	static foreach (field; LogicalFields!TModel)
	{
		static if (is(typeof(__traits(getMember, TModel, field)))
			&& !isCallable!(__traits(getMember, TModel, field)))
		{
			processField!(TModel, field, field)(serialized);
		}
	}

	return serialized;
}();

enum DormFields(TModel : Model) = DormLayout!TModel.fields;

enum DormField(TModel : Model, string sourceName) = findField(DormFields!TModel, sourceName, TModel.stringof);

private auto findField(ModelFormat.Field[] fields, string name, string modelName)
{
	foreach (ref field; fields)
		if (field.sourceColumn == name)
			return field;
	assert(false, "field " ~ name ~ " not found in model " ~ modelName);
}

template LogicalFields(TModel)
{
	static if (is(TModel : Model))
		alias Classes = AliasSeq!(TModel, BaseClassesTuple!TModel);
	else
		alias Classes = AliasSeq!(TModel);
	alias LogicalFields = AliasSeq!();

	static foreach_reverse (Class; Classes)
		LogicalFields = AliasSeq!(LogicalFields, __traits(derivedMembers, Class));
}

private void processField(TModel, string fieldName, string directFieldName)(ref ModelFormat serialized)
{
	import uda = dorm.annotations;

	alias fieldAlias = __traits(getMember, TModel, directFieldName);

	alias attributes = __traits(getAttributes, fieldAlias);

	bool include = true;
	ModelFormat.Field field;

	field.definedAt = SourceLocation(__traits(getLocation, fieldAlias));
	field.columnName = directFieldName.toSnakeCase;
	field.sourceColumn = fieldName;
	field.type = guessDBType!(typeof(fieldAlias));

	bool nullable = false;
	static if (is(typeof(fieldAlias) == Nullable!T, T)
		|| is(typeof(fieldAlias) : Model))
		nullable = true;

	static if (is(typeof(fieldAlias) == enum))
		field.annotations ~= DBAnnotation(Choices([
				__traits(allMembers, typeof(fieldAlias))
			]));

	static foreach (attribute; attributes)
	{
		static if (__traits(isSame, attribute, uda.autoCreateTime))
		{
			field.type = ModelFormat.Field.DBType.timestamp;
			field.annotations ~= DBAnnotation(AnnotationFlag.autoCreateTime);
		}
		else static if (__traits(isSame, attribute, uda.autoUpdateTime))
		{
			field.type = ModelFormat.Field.DBType.timestamp;
			field.annotations ~= DBAnnotation(AnnotationFlag.autoUpdateTime);
		}
		else static if (__traits(isSame, attribute, uda.timestamp))
		{
			field.type = ModelFormat.Field.DBType.timestamp;
		}
		else static if (__traits(isSame, attribute, uda.primaryKey))
		{
			field.annotations ~= DBAnnotation(AnnotationFlag.primaryKey);
		}
		else static if (__traits(isSame, attribute, uda.autoincrement))
		{
			field.annotations ~= DBAnnotation(AnnotationFlag.autoincrement);
		}
		else static if (__traits(isSame, attribute, uda.unique))
		{
			field.annotations ~= DBAnnotation(AnnotationFlag.unique);
		}
		else static if (__traits(isSame, attribute, uda.embedded))
		{
			static if (is(typeof(fieldAlias) == struct))
			{
				serialized.embeddedStructs ~= fieldName;
				alias TSubModel = typeof(fieldAlias);
				static foreach (subfield; LogicalFields!TSubModel)
				{
					static if (is(typeof(__traits(getMember, TSubModel, subfield)))
						&& !isCallable!(__traits(getMember, TSubModel, subfield)))
					{
						processField!(TSubModel, fieldName ~ "." ~ subfield, subfield)(serialized);
					}
				}
			}
			else
				static assert(false, "@embedded is only supported on structs");
			include = false;
		}
		else static if (__traits(isSame, attribute, uda.ignored))
		{
			include = false;
		}
		else static if (__traits(isSame, attribute, uda.notNull))
		{
			nullable = false;
		}
		else static if (is(attribute == constructValue!fn, alias fn))
		{
			field.internalAnnotations ~= InternalAnnotation(ConstructValueRef(
				&makeValueConstructor!(TModel, fieldName, fn)));
		}
		else static if (is(attribute == validator!fn, alias fn))
		{
			field.internalAnnotations ~= InternalAnnotation(ValidatorRef(
				&makeValidator!(TModel, fieldName, fn)));
		}
		else static if (is(typeof(attribute) == maxLength)
			|| is(typeof(attribute) == DefaultValue!T, T)
			|| is(typeof(attribute) == index))
		{
			field.annotations ~= DBAnnotation(attribute);
		}
		else static if (is(typeof(attribute) == Choices))
		{
			field.type = ModelFormat.Field.DBType.choices;
			field.annotations ~= DBAnnotation(attribute);
		}
		else static if (is(typeof(attribute) == columnName))
		{
			field.columnName = attribute.name;
		}
		else static if (isDBAttribute!attribute)
		{
			static assert(false, "Unsupported attribute " ~ attribute.stringof ~ " on " ~ TModel.stringof ~ "." ~ fieldName);
		}
	}

	if (!nullable)
		field.annotations ~= DBAnnotation(AnnotationFlag.notNull);

	if (include)
	{
		if (field.type == InvalidDBType)
			throw new Exception("Cannot resolve DBType from " ~ typeof(fieldAlias).stringof);
		serialized.fields ~= field;
	}
}

private void makeValueConstructor(TModel, string fieldName, alias fn)(Model model)
{
	auto m = cast(TModel) model;
	assert(m, "invalid valueConstructor call: got instance of `" ~ typeid(model).name
		~ "`, but expected `" ~ TModel.stringof ~ '`');
	__traits(getMember, m, fieldName) = fn();
}

private bool makeValidator(TModel, string fieldName, alias fn)(Model model)
{
	import std.functional : unaryFun;

	auto m = cast(TModel) model;
	assert(m, "invalid validator call: got instance of `" ~ typeid(model).name
		~ "`, but expected `" ~ TModel.stringof ~ '`');
	return unaryFun!fn(__traits(getMember, m, fieldName));
}

private string toSnakeCase(string s)
{
	import std.array;
	import std.ascii;

	auto ret = appender!(char[]);
	int upperCount;
	char lastChar;
	foreach (char c; s)
	{
		scope (exit)
			lastChar = c;
		if (upperCount)
		{
			if (isUpper(c))
			{
				ret ~= toLower(c);
				upperCount++;
			}
			else
			{
				if (isDigit(c))
				{
					ret ~= '_';
					ret ~= c;
				}
				else if (isLower(c) && upperCount > 1 && ret.data.length)
				{
					auto last = ret.data[$ - 1];
					ret.shrinkTo(ret.data.length - 1);
					ret ~= '_';
					ret ~= last;
					ret ~= c;
				}
				else
				{
					ret ~= toLower(c);
				}
				upperCount = 0;
			}
		}
		else if (isUpper(c))
		{
			if (ret.data.length
				&& ret.data[$ - 1] != '_'
				&& !lastChar.isUpper)
			{
				ret ~= '_';
				ret ~= toLower(c);
				upperCount++;
			}
			else
			{
				if (ret.data.length)
					upperCount++;
				ret ~= toLower(c);
			}
		}
		else if (c == '_')
		{
			if (ret.data.length)
				ret ~= '_';
		}
		else if (isDigit(c))
		{
			if (ret.data.length && !ret.data[$ - 1].isDigit && ret.data[$ - 1] != '_')
				ret ~= '_';
			ret ~= c;
		}
		else
		{
			if (ret.data.length && ret.data[$ - 1].isDigit && ret.data[$ - 1] != '_')
				ret ~= '_';
			ret ~= c;
		}
	}

	auto slice = ret.data;
	while (slice.length && slice[$ - 1] == '_')
		slice = slice[0 .. $ - 1];
	return slice.idup;
}

unittest
{
	assert("".toSnakeCase == "");
	assert("a".toSnakeCase == "a");
	assert("A".toSnakeCase == "a");
	assert("AB".toSnakeCase == "ab");
	assert("JsonValue".toSnakeCase == "json_value");
	assert("HTTPHandler".toSnakeCase == "http_handler");
	assert("Test123".toSnakeCase == "test_123");
	assert("foo_bar".toSnakeCase == "foo_bar");
	assert("foo_Bar".toSnakeCase == "foo_bar");
	assert("Foo_Bar".toSnakeCase == "foo_bar");
	assert("FOO_bar".toSnakeCase == "foo_bar");
	assert("FOO__bar".toSnakeCase == "foo__bar");
	assert("do_".toSnakeCase == "do");
	assert("fooBar_".toSnakeCase == "foo_bar");
	assert("_do".toSnakeCase == "do");
	assert("_fooBar".toSnakeCase == "foo_bar");
	assert("_FooBar".toSnakeCase == "foo_bar");
	assert("HTTP2".toSnakeCase == "http_2");
	assert("HTTP2Foo".toSnakeCase == "http_2_foo");
	assert("HTTP2foo".toSnakeCase == "http_2_foo");
}

private template isDBAttribute(alias attr)
{
	pragma(msg, "check " ~ attr.stringof);
	enum isDBAttribute = true;
}

private enum InvalidDBType = cast(ModelFormat.Field.DBType)int.max;

private template guessDBType(T)
{
	static if (is(T == enum))
		enum guessDBType = ModelFormat.Field.DBType.choices;
	else static if (is(T == BitFlags!U, U))
		enum guessDBType = ModelFormat.Field.DBType.set;
	else static if (is(T == Nullable!U, U))
	{
		static if (__traits(compiles, guessDBType!U))
			enum guessDBType = guessDBType!U;
		else
			static assert(false, "cannot resolve DBType from nullable " ~ U.stringof);
	}
	else static if (__traits(compiles, guessDBTypeBase!T))
		enum guessDBType = guessDBTypeBase!T;
	else
		enum guessDBType = InvalidDBType;
}

private enum guessDBTypeBase(T : const(char)[]) = ModelFormat.Field.DBType.varchar;
private enum guessDBTypeBase(T : const(ubyte)[]) = ModelFormat.Field.DBType.varbinary;
private enum guessDBTypeBase(T : byte) = ModelFormat.Field.DBType.int8;
private enum guessDBTypeBase(T : short) = ModelFormat.Field.DBType.int16;
private enum guessDBTypeBase(T : int) = ModelFormat.Field.DBType.int32;
private enum guessDBTypeBase(T : long) = ModelFormat.Field.DBType.int64;
private enum guessDBTypeBase(T : ubyte) = ModelFormat.Field.DBType.uint8;
private enum guessDBTypeBase(T : ushort) = ModelFormat.Field.DBType.uint16;
private enum guessDBTypeBase(T : uint) = ModelFormat.Field.DBType.uint32;
private enum guessDBTypeBase(T : float) = ModelFormat.Field.DBType.floatNumber;
private enum guessDBTypeBase(T : double) = ModelFormat.Field.DBType.doubleNumber;
private enum guessDBTypeBase(T : bool) = ModelFormat.Field.DBType.boolean;
private enum guessDBTypeBase(T : Date) = ModelFormat.Field.DBType.date;
private enum guessDBTypeBase(T : DateTime) = ModelFormat.Field.DBType.datetime;
private enum guessDBTypeBase(T : SysTime) = ModelFormat.Field.DBType.timestamp;
private enum guessDBTypeBase(T : TimeOfDay) = ModelFormat.Field.DBType.time;

unittest
{
	import std.sumtype;

	struct Mod
	{
		import std.datetime;
		import std.typecons;

		enum State : string
		{
			ok = "ok",
			warn = "warn",
			critical = "critical",
			unknown = "unknown"
		}

		class User : Model
		{
			@maxLength(255)
			string username;

			@maxLength(255)
			string password;

			@maxLength(255)
			Nullable!string email;

			ubyte age;

			Nullable!DateTime birthday;

			@autoCreateTime
			SysTime createdAt;

			@autoUpdateTime
			Nullable!SysTime updatedAt;

			@autoCreateTime
			ulong createdAt2;

			@autoUpdateTime
			Nullable!ulong updatedAt2;

			State state;

			@choices("ok", "warn", "critical", "unknown")
			string state2;

			@columnName("admin")
			bool isAdmin;

			@constructValue!(() => Clock.currTime + 4.hours)
			SysTime validUntil;

			@maxLength(255)
			@defaultValue("")
			string comment;

			@defaultValue(1337)
			int counter;

			@primaryKey
			long ownPrimaryKey;

			@timestamp
			ulong creationTime;

			@unique
			int uuid;

			@validator!(x => x >= 18)
			int someInt;

			@ignored
			int imNotIncluded;
		}
	}

	auto mod = processModelsToDeclarations!Mod;
	assert(mod.models.length == 1);

	auto m = mod.models[0];

	// Length is always len(m.fields + 1) as dorm.model.Model adds the id field
	assert(m.fields.length == 20);

	// field[0] gets added by dorm.model.Model
	assert(m.fields[0].columnName == "id");
	assert(m.fields[0].type == ModelFormat.Field.DBType.int64);
	assert(m.fields[0].annotations == [DBAnnotation(AnnotationFlag.autoincrement), DBAnnotation(AnnotationFlag.primaryKey), DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[1].columnName == "username");
	assert(m.fields[1].type == ModelFormat.Field.DBType.varchar);
	assert(m.fields[1].annotations == [DBAnnotation(maxLength(255)), DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[2].columnName == "password");
	assert(m.fields[2].type == ModelFormat.Field.DBType.varchar);
	assert(m.fields[2].annotations == [DBAnnotation(maxLength(255)), DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[3].columnName == "email");
	assert(m.fields[3].type == ModelFormat.Field.DBType.varchar);
	assert(m.fields[3].annotations == [DBAnnotation(maxLength(255))]);

	assert(m.fields[4].columnName == "age");
	assert(m.fields[4].type == ModelFormat.Field.DBType.uint8);
	assert(m.fields[4].annotations == [DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[5].columnName == "birthday");
	assert(m.fields[5].type == ModelFormat.Field.DBType.datetime);
	assert(m.fields[5].annotations == []);

	assert(m.fields[6].columnName == "created_at");
	assert(m.fields[6].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[6].annotations == [
			DBAnnotation(AnnotationFlag.autoCreateTime),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[7].columnName == "updated_at");
	assert(m.fields[7].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[7].annotations == [
			DBAnnotation(AnnotationFlag.autoUpdateTime)
		]);

	assert(m.fields[8].columnName == "created_at_2");
	assert(m.fields[8].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[8].annotations == [
			DBAnnotation(AnnotationFlag.autoCreateTime),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[9].columnName == "updated_at_2");
	assert(m.fields[9].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[9].annotations == [
			DBAnnotation(AnnotationFlag.autoUpdateTime)
		]);

	assert(m.fields[10].columnName == "state");
	assert(m.fields[10].type == ModelFormat.Field.DBType.choices);
	assert(m.fields[10].annotations == [
			DBAnnotation(Choices(["ok", "warn", "critical", "unknown"])),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[11].columnName == "state_2");
	assert(m.fields[11].type == ModelFormat.Field.DBType.choices);
	assert(m.fields[11].annotations == [
			DBAnnotation(Choices(["ok", "warn", "critical", "unknown"])),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[12].columnName == "admin");
	assert(m.fields[12].type == ModelFormat.Field.DBType.boolean);
	assert(m.fields[12].annotations == [DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[13].columnName == "valid_until");
	assert(m.fields[13].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[13].annotations == [
			DBAnnotation(AnnotationFlag.notNull)
		]);
	assert(m.fields[13].internalAnnotations.length == 1);
	assert(m.fields[13].internalAnnotations[0].match!((ConstructValueRef r) => true, _ => false));

	assert(m.fields[14].columnName == "comment");
	assert(m.fields[14].type == ModelFormat.Field.DBType.varchar);
	assert(m.fields[14].annotations == [
			DBAnnotation(maxLength(255)),
			DBAnnotation(defaultValue("")),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[15].columnName == "counter");
	assert(m.fields[15].type == ModelFormat.Field.DBType.int32);
	assert(m.fields[15].annotations == [
			DBAnnotation(defaultValue(1337)),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[16].columnName == "own_primary_key");
	assert(m.fields[16].type == ModelFormat.Field.DBType.int64);
	assert(m.fields[16].annotations == [
			DBAnnotation(AnnotationFlag.primaryKey),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[17].columnName == "creation_time");
	assert(m.fields[17].type == ModelFormat.Field.DBType.timestamp);
	assert(m.fields[17].annotations == [DBAnnotation(AnnotationFlag.notNull)]);

	assert(m.fields[18].columnName == "uuid");
	assert(m.fields[18].type == ModelFormat.Field.DBType.int32);
	assert(m.fields[18].annotations == [
			DBAnnotation(AnnotationFlag.unique),
			DBAnnotation(AnnotationFlag.notNull)
		]);

	assert(m.fields[19].columnName == "some_int");
	assert(m.fields[19].type == ModelFormat.Field.DBType.int32);
	assert(m.fields[19].annotations == [
			DBAnnotation(AnnotationFlag.notNull)
		]);
	assert(m.fields[19].internalAnnotations.length == 1);
	assert(m.fields[19].internalAnnotations[0].match!((ValidatorRef r) => true, _ => false));
}

unittest
{
	struct Mod
	{
		abstract class NamedThing : Model
		{
			@maxLength(255)
			string name;
		}

		class User : NamedThing
		{
			int age;
		}
	}

	auto mod = processModelsToDeclarations!Mod;
	assert(mod.models.length == 1);

	auto m = mod.models[0];
	assert(m.tableName == "user");
	// As Model also adds the id field, length is 3
	assert(m.fields.length == 3);
	assert(m.fields[0].columnName == "id");
	assert(m.fields[1].columnName == "name");
	assert(m.fields[2].columnName == "age");
}

unittest
{
	struct Mod
	{
		struct SuperCommon
		{
			int superCommonField;
		}

		struct Common
		{
			string commonName;
			@embedded
			SuperCommon superCommon;
		}

		class NamedThing : Model
		{
			@embedded
			Common common;

			@maxLength(255)
			string name;
		}
	}

	auto mod = processModelsToDeclarations!Mod;
	assert(mod.models.length == 1);
	auto m = mod.models[0];
	assert(m.tableName == "named_thing");
	// As Model also adds the id field, length is 3
	assert(m.fields.length == 4);
	assert(m.fields[1].columnName == "common_name");
	assert(m.fields[1].sourceColumn == "common.commonName");
	assert(m.fields[2].columnName == "super_common_field");
	assert(m.fields[2].sourceColumn == "common.superCommon.superCommonField");
	assert(m.fields[3].columnName == "name");
	assert(m.fields[3].sourceColumn == "name");
	assert(m.embeddedStructs == ["common", "common.superCommon"]);
}
