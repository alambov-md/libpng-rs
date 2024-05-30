package com.alambov.androidtests

import android.Manifest
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.runner.permission.PermissionRequester
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder
import org.junit.runner.RunWith
import java.io.File
import java.io.FileReader


/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {
    @get:Rule
    var folder = TemporaryFolder()

    @Test
    fun testPngRead() {
        val pngAsset = InstrumentationRegistry.getInstrumentation().context.assets.open("test.png")
        val pngBytes = pngAsset.readBytes()

        var code = MainActivity.testReadPngFromMemory(pngBytes)
        assertEquals(1, code)
    }
}