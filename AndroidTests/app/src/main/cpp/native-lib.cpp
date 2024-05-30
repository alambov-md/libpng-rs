#include <jni.h>
#include <string>
#include <cstdio>
#include <unistd.h>

#include "binding.h"

extern "C" JNIEXPORT jint

JNICALL
Java_com_alambov_androidtests_MainActivity_00024Companion_testReadFromPngFileToMemoryJNI(
        JNIEnv *env,
        jobject /* this */,
        jstring filePathStr) {
    const jchar *file_path = env->GetStringChars(filePathStr, 0);

    return test_read_from_png_file_to_memory((const char *)file_path);
}

extern "C" JNIEXPORT jint

JNICALL
Java_com_alambov_androidtests_MainActivity_00024Companion_testReadPngFromMemoryJNI(
        JNIEnv *env,
        jobject /* this */,
        jbyteArray byteArray) {
    jbyte *bytes = env->GetByteArrayElements(byteArray, 0);

    return test_read_png_from_memory((const void *)bytes, env->GetArrayLength(byteArray));
}