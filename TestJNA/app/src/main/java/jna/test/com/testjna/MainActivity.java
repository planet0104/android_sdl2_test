package jna.test.com.testjna;

import android.app.Activity;
import android.os.Bundle;

public class MainActivity extends Activity {
    static final String TAG = MainActivity.class.getSimpleName();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        System.out.println(Lib.start("呵呵"));
    }
}
