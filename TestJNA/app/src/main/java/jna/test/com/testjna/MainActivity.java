package jna.test.com.testjna;

import android.Manifest;
import android.app.Activity;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.util.Log;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.Buffer;
import java.nio.ByteBuffer;
import java.util.ArrayList;

import static android.os.Environment.DIRECTORY_PICTURES;

public class MainActivity extends Activity {
    static final String TAG = MainActivity.class.getSimpleName();
    public interface Game extends Library {
        Pointer start();
        float test();
        void end();
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        requestPermissions(new String[]{Manifest.permission.WRITE_EXTERNAL_STORAGE}, 0);

        //Native.loadLibrary()
        File file_dir = getFilesDir();
        File android_aarch64 = new File(file_dir, "arm64-v8a");
        if(!android_aarch64.exists()){
            android_aarch64.mkdir();
        }
        final File so = new File(android_aarch64, "libSDL2.so");
        if(so.exists()){
            so.delete();
        }
        try{
            InputStream s = getAssets().open("libs/arm64-v8a/libSDL2.so");
            inputStreamToFile(s, so);
        }catch (Exception e){
            e.printStackTrace();
        }
        Log.d(TAG, "so文件存在? "+so.exists()+" "+so.getAbsolutePath());

        Log.d(TAG, "Build.CPU_ABI="+ Build.CPU_ABI);

        final Game game = Native.loadLibrary("main", Game.class);
        Pointer pointer = game.start();
        int len = pointer.getInt(0);
        Log.d(TAG, "长度:"+len);
        byte[] buffer = pointer.getByteArray(4, 4);
        Log.d(TAG, "字节1==="+buffer[0]);
        Log.d(TAG, "字节2==="+buffer[1]);
        Log.d(TAG, "字节3==="+buffer[2]);
        Log.d(TAG, "字节4==="+buffer[3]);

    }

    public void inputStreamToFile(InputStream ins,File file) throws IOException {
        OutputStream os = new FileOutputStream(file);
        int bytesRead = 0;
        byte[] buffer = new byte[8192];
        while ((bytesRead = ins.read(buffer, 0, 8192)) != -1) {
            os.write(buffer, 0, bytesRead);
        }
        os.close();
        ins.close();
    }

//    public void inputStreamToFile(InputStream ins,File file) throws IOException {
//        OutputStream os = new FileOutputStream(file);
//        int bytesRead = 0;
//        byte[] buffer = new byte[8192];
//        while ((bytesRead = ins.read(buffer, 0, 8192)) != -1) {
//            os.write(buffer, 0, bytesRead);
//        }
//        os.close();
//        ins.close();
//    }
}
