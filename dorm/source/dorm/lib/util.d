module dorm.lib.util;

import core.sync.event;
import std.typecons;
import std.traits;

import dorm.lib.ffi;

struct FreeableAsyncResult(T)
{
	Event event;
	T raw_result;
	Exception error;

	static AsyncResult make()
	{
		AsyncResult res;
		res.event = Event(true, false);
		return res;
	}

	alias Callback = extern(C) void function(void* data, FFIResult!T result) nothrow;

	Tuple!(Callback, void*) callback() return
	{
		extern(C) static void ret(void* data, FFIResult!T result) nothrow
		{
			auto res = cast(FFIResult*)data;
			if (result.error.size)
				res.error = new Exception(result.error.data.idup);
			else
				res.result = result.raw_result;
			res.event.set();
		}

		return tuple(&ret, cast(void*)&this);
	}

	T result()
	{
		event.wait();
		if (error)
			throw error;
		return raw_result;
	}
}

Event* async_call(alias fn)(Parameters!fn[0 .. $ - 2] args, void delegate(scope Parameters!(Parameters!fn[$ - 1])[1 .. $]) callback)
{
	import core.stdc.stdlib;
	import core.memory;

	Event* ret = new Event(true, false);
	auto data = malloc(callback.sizeof + size_t.sizeof);
	*(cast(typeof(callback)*)data) = callback;
	*(cast(typeof(callback)*)(data + callback.sizeof)) = ret;
	GC.addRoot(callback.ptr);
	GC.addRoot(ret);
	extern(C) static void callback(Parameters!(Parameters!fn[$ - 1]) args) nothrow
	{
		auto event = *cast(Event**)(args[0] + callback.sizeof);
		auto dg = *cast(typeof(callback)*)args[0];
		dg(forward!(args[1 .. $]));
		event.set();
		GC.removeRoot(event);
		GC.removeRoot(dg.ptr);
		free(args[0]);
	}
	fn(forward!args, &callback, data);
	return ret;
}
