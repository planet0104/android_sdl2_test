package jna.test.com.testjna;

public class Lib {
    static{
        System.loadLibrary("paint");
    }
    public static native String start(String msg);
}
