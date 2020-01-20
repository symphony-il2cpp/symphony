LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

LOCAL_MODULE     := libil2cpp
LOCAL_C_INCLUDES := wrapper.hpp
LOCAL_CFLAGS     := -Ivendor/libil2cpp -Wall -Wextra

include $(BUILD_STATIC_LIBRARY)
