package jna.test.com.testjna;

import android.Manifest;
import android.app.Activity;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.util.Log;

import com.sun.jna.Library;
import com.sun.jna.Native;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

import static android.os.Environment.DIRECTORY_PICTURES;

public class MainActivity extends Activity {
    static final String TAG = MainActivity.class.getSimpleName();
    public interface Game extends Library {
        int start(String libpath);
        float test();
        void end();
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        //System.load("SDL2");
        //System.loadLibrary("SDL2");
//        String path=getApplicationContext().getPackageResourcePath();
//        System.out.println("String path=getApplicationContext().getPackageResourcePath()="+path);
//        File file = new File("/data/data/jna.test.com.testjna/lib/");
//        String[] list = file.list();
//        for(String l : list){
//            System.out.println("文件:"+l);
//        }
        requestPermissions(new String[]{Manifest.permission.WRITE_EXTERNAL_STORAGE}, 0);
        File file = Environment.getExternalStoragePublicDirectory(DIRECTORY_PICTURES);
        File bmp = new File(file, "test.bmp");
        Log.d(TAG, bmp.getAbsolutePath());
        try {
            FileOutputStream f = new FileOutputStream(bmp);
            f.write(11);
            f.flush();
            f.close();
            Log.d(TAG, "文件写入成功!");
        } catch (Exception e) {
            e.printStackTrace();
        }

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

//        NativeLibrary.addSearchPath("main", file_dir.getAbsolutePath());
//        f = NativeLibrary.getInstance("main").getFile();
//        Log.d(TAG, "f="+f);
        final Game game = Native.loadLibrary("main", Game.class);
        Log.d(TAG, "step1.");
        float get = game.test();
        Log.d(TAG, "get="+get);
        Log.d(TAG, "哈哈哈>>>>>>>>>>>111");
        try{
            Log.d(TAG, "哈哈哈>>>>>>>>>>>2222");
            int result = game.start(so.getAbsolutePath());
            Log.d(TAG, "result>>>>>>>>>>>"+result);
        }catch (Throwable e){
            Log.d(TAG, "哈哈哈>>>>>>>>>>>4444");
            e.printStackTrace();
        }finally {
            Log.d(TAG, "呵呵呵呵 Finally!!!");
        }
        Log.d(TAG, "哈哈哈>>>>>>>>>>>5555");


//        File file = new File(getExternalCacheDir(), "libcompute.so");
//        try {
//            InputStream fis = getAssets().open("libcompute.so");
//            inputStreamToFile(fis, file);
//
//            Log.d(TAG, "file.exist="+file.exists()+" path="+file.getAbsolutePath()+" absolute="+file.isAbsolute());
//
//            Game compute = Native.loadLibrary(file.getAbsolutePath(), Game.class);
//
//            //Game compute = Native.loadLibrary("compute", Game.class);
//
//            Log.d("HelloJNA","计算结果:"+compute.compute(1.0f));
//        } catch (Throwable e) {
//            Log.e("helloJNA", "出错:"+e.getMessage());
//            e.printStackTrace();
//        }
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
