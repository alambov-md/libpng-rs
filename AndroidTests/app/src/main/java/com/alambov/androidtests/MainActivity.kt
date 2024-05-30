package com.alambov.androidtests

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView
import com.alambov.androidtests.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
    }
    companion object {
        // Used to load the 'androidtests' library on application startup.
        init {
            System.loadLibrary("androidtests")
        }

        public fun testReadPngFromMemory(byteArray: ByteArray): Int {
            return testReadPngFromMemoryJNI(byteArray)
        }
        private external fun testReadPngFromMemoryJNI(byteArray: ByteArray): Int
    }
}