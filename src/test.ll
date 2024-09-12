; ModuleID = 'hello'
source_filename = "hello"
target triple = "x86_64-pc-linux-gnu"

declare i32 @puts(i8*)

declare i64 @write(i32, i8*, i64)

declare i32 @vkCreateInstance({ i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** }*, i8*, i8**)

declare i32 @vkEnumeratePhysicalDevices(i8*, i32*, i8**)

declare i32 @vkCreateDevice(i8*, { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* }*, i8*, i8**)

declare void @glfwWindowHint(i32, i32)

declare i8* @glfwCreateWindow(i32, i32, i8*, i8*, i8*)

declare void @glfwPollEvents()

declare void @glfwShowWindow(i8*)

define i32 @main() {
entry:
  %vert_spv = alloca { i64, i32* }, align 8
  %arr = alloca i32, i32 135, align 4
  %GLFW_NO_API = alloca i32, align 4
  %GLFW_TRUE = alloca i32, align 4
  %GLFW_VISIBLE = alloca i32, align 4
  %GLFW_CLIENT_API = alloca i32, align 4
  %window = alloca i8*, align 8
  %arr1 = alloca i8, i32 14, align 1
  %app_info = alloca { i32, i8*, i8*, i32, i8*, i32, i32 }, align 8
  %arr2 = alloca i8, i32 6, align 1
  %arr3 = alloca i8, i32 7, align 1
  %createinfo = alloca { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** }, align 8
  %arr4 = alloca i8*, align 8
  %arr5 = alloca i8, i32 28, align 1
  %instance = alloca i8*, align 8
  %physical_device_count = alloca i32, align 4
  %physical_device = alloca i8*, align 8
  %priority = alloca float, align 4
  %device_queue_createinfo = alloca { i32, i32, i32, float* }, align 8
  %device_create_info = alloca { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* }, align 8
  %device = alloca i8*, align 8
  %x = alloca i32, align 4
  br label %body

body:                                             ; preds = %entry
  %0 = getelementptr i32, i32* %arr, i32 0
  store i32 119734787, i32* %0, align 4
  %1 = getelementptr i32, i32* %arr, i32 1
  store i32 65536, i32* %1, align 4
  %2 = getelementptr i32, i32* %arr, i32 2
  store i32 0, i32* %2, align 4
  %3 = getelementptr i32, i32* %arr, i32 3
  store i32 27, i32* %3, align 4
  %4 = getelementptr i32, i32* %arr, i32 4
  store i32 0, i32* %4, align 4
  %5 = getelementptr i32, i32* %arr, i32 5
  store i32 131089, i32* %5, align 4
  %6 = getelementptr i32, i32* %arr, i32 6
  store i32 1, i32* %6, align 4
  %7 = getelementptr i32, i32* %arr, i32 7
  store i32 196622, i32* %7, align 4
  %8 = getelementptr i32, i32* %arr, i32 8
  store i32 0, i32* %8, align 4
  %9 = getelementptr i32, i32* %arr, i32 9
  store i32 0, i32* %9, align 4
  %10 = getelementptr i32, i32* %arr, i32 10
  store i32 393221, i32* %10, align 4
  %11 = getelementptr i32, i32* %arr, i32 11
  store i32 1, i32* %11, align 4
  %12 = getelementptr i32, i32* %arr, i32 12
  store i32 1600939891, i32* %12, align 4
  %13 = getelementptr i32, i32* %arr, i32 13
  store i32 1953654134, i32* %13, align 4
  %14 = getelementptr i32, i32* %arr, i32 14
  store i32 1936683103, i32* %14, align 4
  %15 = getelementptr i32, i32* %arr, i32 15
  store i32 0, i32* %15, align 4
  %16 = getelementptr i32, i32* %arr, i32 16
  store i32 262215, i32* %16, align 4
  %17 = getelementptr i32, i32* %arr, i32 17
  store i32 1, i32* %17, align 4
  %18 = getelementptr i32, i32* %arr, i32 18
  store i32 11, i32* %18, align 4
  %19 = getelementptr i32, i32* %arr, i32 19
  store i32 0, i32* %19, align 4
  %20 = getelementptr i32, i32* %arr, i32 20
  store i32 196630, i32* %20, align 4
  %21 = getelementptr i32, i32* %arr, i32 21
  store i32 2, i32* %21, align 4
  %22 = getelementptr i32, i32* %arr, i32 22
  store i32 32, i32* %22, align 4
  %23 = getelementptr i32, i32* %arr, i32 23
  store i32 262167, i32* %23, align 4
  %24 = getelementptr i32, i32* %arr, i32 24
  store i32 3, i32* %24, align 4
  %25 = getelementptr i32, i32* %arr, i32 25
  store i32 2, i32* %25, align 4
  %26 = getelementptr i32, i32* %arr, i32 26
  store i32 4, i32* %26, align 4
  %27 = getelementptr i32, i32* %arr, i32 27
  store i32 262176, i32* %27, align 4
  %28 = getelementptr i32, i32* %arr, i32 28
  store i32 4, i32* %28, align 4
  %29 = getelementptr i32, i32* %arr, i32 29
  store i32 3, i32* %29, align 4
  %30 = getelementptr i32, i32* %arr, i32 30
  store i32 3, i32* %30, align 4
  %31 = getelementptr i32, i32* %arr, i32 31
  store i32 262203, i32* %31, align 4
  %32 = getelementptr i32, i32* %arr, i32 32
  store i32 4, i32* %32, align 4
  %33 = getelementptr i32, i32* %arr, i32 33
  store i32 5, i32* %33, align 4
  %34 = getelementptr i32, i32* %arr, i32 34
  store i32 3, i32* %34, align 4
  %35 = getelementptr i32, i32* %arr, i32 35
  store i32 262149, i32* %35, align 4
  %36 = getelementptr i32, i32* %arr, i32 36
  store i32 6, i32* %36, align 4
  %37 = getelementptr i32, i32* %arr, i32 37
  store i32 1885302377, i32* %37, align 4
  %38 = getelementptr i32, i32* %arr, i32 38
  store i32 29551, i32* %38, align 4
  %39 = getelementptr i32, i32* %arr, i32 39
  store i32 262215, i32* %39, align 4
  %40 = getelementptr i32, i32* %arr, i32 40
  store i32 6, i32* %40, align 4
  %41 = getelementptr i32, i32* %arr, i32 41
  store i32 30, i32* %41, align 4
  %42 = getelementptr i32, i32* %arr, i32 42
  store i32 0, i32* %42, align 4
  %43 = getelementptr i32, i32* %arr, i32 43
  store i32 262167, i32* %43, align 4
  %44 = getelementptr i32, i32* %arr, i32 44
  store i32 7, i32* %44, align 4
  %45 = getelementptr i32, i32* %arr, i32 45
  store i32 2, i32* %45, align 4
  %46 = getelementptr i32, i32* %arr, i32 46
  store i32 2, i32* %46, align 4
  %47 = getelementptr i32, i32* %arr, i32 47
  store i32 262176, i32* %47, align 4
  %48 = getelementptr i32, i32* %arr, i32 48
  store i32 8, i32* %48, align 4
  %49 = getelementptr i32, i32* %arr, i32 49
  store i32 1, i32* %49, align 4
  %50 = getelementptr i32, i32* %arr, i32 50
  store i32 7, i32* %50, align 4
  %51 = getelementptr i32, i32* %arr, i32 51
  store i32 262203, i32* %51, align 4
  %52 = getelementptr i32, i32* %arr, i32 52
  store i32 8, i32* %52, align 4
  %53 = getelementptr i32, i32* %arr, i32 53
  store i32 9, i32* %53, align 4
  %54 = getelementptr i32, i32* %arr, i32 54
  store i32 1, i32* %54, align 4
  %55 = getelementptr i32, i32* %arr, i32 55
  store i32 262149, i32* %55, align 4
  %56 = getelementptr i32, i32* %arr, i32 56
  store i32 10, i32* %56, align 4
  %57 = getelementptr i32, i32* %arr, i32 57
  store i32 1601467759, i32* %57, align 4
  %58 = getelementptr i32, i32* %arr, i32 58
  store i32 7565168, i32* %58, align 4
  %59 = getelementptr i32, i32* %arr, i32 59
  store i32 262215, i32* %59, align 4
  %60 = getelementptr i32, i32* %arr, i32 60
  store i32 10, i32* %60, align 4
  %61 = getelementptr i32, i32* %arr, i32 61
  store i32 30, i32* %61, align 4
  %62 = getelementptr i32, i32* %arr, i32 62
  store i32 0, i32* %62, align 4
  %63 = getelementptr i32, i32* %arr, i32 63
  store i32 262176, i32* %63, align 4
  %64 = getelementptr i32, i32* %arr, i32 64
  store i32 11, i32* %64, align 4
  %65 = getelementptr i32, i32* %arr, i32 65
  store i32 3, i32* %65, align 4
  %66 = getelementptr i32, i32* %arr, i32 66
  store i32 7, i32* %66, align 4
  %67 = getelementptr i32, i32* %arr, i32 67
  store i32 262203, i32* %67, align 4
  %68 = getelementptr i32, i32* %arr, i32 68
  store i32 11, i32* %68, align 4
  %69 = getelementptr i32, i32* %arr, i32 69
  store i32 12, i32* %69, align 4
  %70 = getelementptr i32, i32* %arr, i32 70
  store i32 3, i32* %70, align 4
  %71 = getelementptr i32, i32* %arr, i32 71
  store i32 131091, i32* %71, align 4
  %72 = getelementptr i32, i32* %arr, i32 72
  store i32 13, i32* %72, align 4
  %73 = getelementptr i32, i32* %arr, i32 73
  store i32 131091, i32* %73, align 4
  %74 = getelementptr i32, i32* %arr, i32 74
  store i32 14, i32* %74, align 4
  %75 = getelementptr i32, i32* %arr, i32 75
  store i32 196641, i32* %75, align 4
  %76 = getelementptr i32, i32* %arr, i32 76
  store i32 15, i32* %76, align 4
  %77 = getelementptr i32, i32* %arr, i32 77
  store i32 14, i32* %77, align 4
  %78 = getelementptr i32, i32* %arr, i32 78
  store i32 327734, i32* %78, align 4
  %79 = getelementptr i32, i32* %arr, i32 79
  store i32 13, i32* %79, align 4
  %80 = getelementptr i32, i32* %arr, i32 80
  store i32 16, i32* %80, align 4
  %81 = getelementptr i32, i32* %arr, i32 81
  store i32 0, i32* %81, align 4
  %82 = getelementptr i32, i32* %arr, i32 82
  store i32 15, i32* %82, align 4
  %83 = getelementptr i32, i32* %arr, i32 83
  store i32 262205, i32* %83, align 4
  %84 = getelementptr i32, i32* %arr, i32 84
  store i32 7, i32* %84, align 4
  %85 = getelementptr i32, i32* %arr, i32 85
  store i32 17, i32* %85, align 4
  %86 = getelementptr i32, i32* %arr, i32 86
  store i32 9, i32* %86, align 4
  %87 = getelementptr i32, i32* %arr, i32 87
  store i32 327761, i32* %87, align 4
  %88 = getelementptr i32, i32* %arr, i32 88
  store i32 2, i32* %88, align 4
  %89 = getelementptr i32, i32* %arr, i32 89
  store i32 18, i32* %89, align 4
  %90 = getelementptr i32, i32* %arr, i32 90
  store i32 17, i32* %90, align 4
  %91 = getelementptr i32, i32* %arr, i32 91
  store i32 0, i32* %91, align 4
  %92 = getelementptr i32, i32* %arr, i32 92
  store i32 262205, i32* %92, align 4
  %93 = getelementptr i32, i32* %arr, i32 93
  store i32 7, i32* %93, align 4
  %94 = getelementptr i32, i32* %arr, i32 94
  store i32 19, i32* %94, align 4
  %95 = getelementptr i32, i32* %arr, i32 95
  store i32 9, i32* %95, align 4
  %96 = getelementptr i32, i32* %arr, i32 96
  store i32 327761, i32* %96, align 4
  %97 = getelementptr i32, i32* %arr, i32 97
  store i32 2, i32* %97, align 4
  %98 = getelementptr i32, i32* %arr, i32 98
  store i32 20, i32* %98, align 4
  %99 = getelementptr i32, i32* %arr, i32 99
  store i32 19, i32* %99, align 4
  %100 = getelementptr i32, i32* %arr, i32 100
  store i32 1, i32* %100, align 4
  %101 = getelementptr i32, i32* %arr, i32 101
  store i32 196630, i32* %101, align 4
  %102 = getelementptr i32, i32* %arr, i32 102
  store i32 21, i32* %102, align 4
  %103 = getelementptr i32, i32* %arr, i32 103
  store i32 32, i32* %103, align 4
  %104 = getelementptr i32, i32* %arr, i32 104
  store i32 262187, i32* %104, align 4
  %105 = getelementptr i32, i32* %arr, i32 105
  store i32 21, i32* %105, align 4
  %106 = getelementptr i32, i32* %arr, i32 106
  store i32 22, i32* %106, align 4
  %107 = getelementptr i32, i32* %arr, i32 107
  store i32 0, i32* %107, align 4
  %108 = getelementptr i32, i32* %arr, i32 108
  store i32 196630, i32* %108, align 4
  %109 = getelementptr i32, i32* %arr, i32 109
  store i32 23, i32* %109, align 4
  %110 = getelementptr i32, i32* %arr, i32 110
  store i32 32, i32* %110, align 4
  %111 = getelementptr i32, i32* %arr, i32 111
  store i32 262187, i32* %111, align 4
  %112 = getelementptr i32, i32* %arr, i32 112
  store i32 23, i32* %112, align 4
  %113 = getelementptr i32, i32* %arr, i32 113
  store i32 24, i32* %113, align 4
  %114 = getelementptr i32, i32* %arr, i32 114
  store i32 1065353216, i32* %114, align 4
  %115 = getelementptr i32, i32* %arr, i32 115
  store i32 458832, i32* %115, align 4
  %116 = getelementptr i32, i32* %arr, i32 116
  store i32 3, i32* %116, align 4
  %117 = getelementptr i32, i32* %arr, i32 117
  store i32 25, i32* %117, align 4
  %118 = getelementptr i32, i32* %arr, i32 118
  store i32 18, i32* %118, align 4
  %119 = getelementptr i32, i32* %arr, i32 119
  store i32 20, i32* %119, align 4
  %120 = getelementptr i32, i32* %arr, i32 120
  store i32 22, i32* %120, align 4
  %121 = getelementptr i32, i32* %arr, i32 121
  store i32 24, i32* %121, align 4
  %122 = getelementptr i32, i32* %arr, i32 122
  store i32 196670, i32* %122, align 4
  %123 = getelementptr i32, i32* %arr, i32 123
  store i32 9, i32* %123, align 4
  %124 = getelementptr i32, i32* %arr, i32 124
  store i32 25, i32* %124, align 4
  %125 = getelementptr i32, i32* %arr, i32 125
  store i32 65536, i32* %125, align 4
  %126 = getelementptr i32, i32* %arr, i32 126
  store i32 262205, i32* %126, align 4
  %127 = getelementptr i32, i32* %arr, i32 127
  store i32 7, i32* %127, align 4
  %128 = getelementptr i32, i32* %arr, i32 128
  store i32 26, i32* %128, align 4
  %129 = getelementptr i32, i32* %arr, i32 129
  store i32 9, i32* %129, align 4
  %130 = getelementptr i32, i32* %arr, i32 130
  store i32 196670, i32* %130, align 4
  %131 = getelementptr i32, i32* %arr, i32 131
  store i32 12, i32* %131, align 4
  %132 = getelementptr i32, i32* %arr, i32 132
  store i32 26, i32* %132, align 4
  %133 = getelementptr i32, i32* %arr, i32 133
  store i32 65536, i32* %133, align 4
  %134 = getelementptr i32, i32* %arr, i32 134
  store i32 65592, i32* %134, align 4
  %135 = insertvalue { i64, i32* } { i64 135, i32* null }, i32* %arr, 1
  store { i64, i32* } %135, { i64, i32* }* %vert_spv, align 8
  store i32 0, i32* %GLFW_NO_API, align 4
  store i32 1, i32* %GLFW_TRUE, align 4
  store i32 0, i32* %GLFW_VISIBLE, align 4
  store i32 0, i32* %GLFW_CLIENT_API, align 4
  %136 = load i32, i32* %GLFW_CLIENT_API, align 4
  %137 = load i32, i32* %GLFW_NO_API, align 4
  call void @glfwWindowHint(i32 %136, i32 %137)
  %138 = getelementptr i8, i8* %arr1, i32 0
  store i8 72, i8* %138, align 1
  %139 = getelementptr i8, i8* %arr1, i32 1
  store i8 101, i8* %139, align 1
  %140 = getelementptr i8, i8* %arr1, i32 2
  store i8 108, i8* %140, align 1
  %141 = getelementptr i8, i8* %arr1, i32 3
  store i8 108, i8* %141, align 1
  %142 = getelementptr i8, i8* %arr1, i32 4
  store i8 111, i8* %142, align 1
  %143 = getelementptr i8, i8* %arr1, i32 5
  store i8 44, i8* %143, align 1
  %144 = getelementptr i8, i8* %arr1, i32 6
  store i8 32, i8* %144, align 1
  %145 = getelementptr i8, i8* %arr1, i32 7
  store i8 87, i8* %145, align 1
  %146 = getelementptr i8, i8* %arr1, i32 8
  store i8 111, i8* %146, align 1
  %147 = getelementptr i8, i8* %arr1, i32 9
  store i8 114, i8* %147, align 1
  %148 = getelementptr i8, i8* %arr1, i32 10
  store i8 108, i8* %148, align 1
  %149 = getelementptr i8, i8* %arr1, i32 11
  store i8 100, i8* %149, align 1
  %150 = getelementptr i8, i8* %arr1, i32 12
  store i8 33, i8* %150, align 1
  %151 = getelementptr i8, i8* %arr1, i32 13
  store i8 0, i8* %151, align 1
  %152 = insertvalue { i64, i8* } { i64 14, i8* null }, i8* %arr1, 1
  %153 = extractvalue { i64, i8* } %152, 1
  %154 = call i8* @glfwCreateWindow(i32 640, i32 480, i8* %153, i8* null, i8* null)
  store i8* %154, i8** %window, align 8
  br i1 false, label %do, label %done

do:                                               ; preds = %do, %body
  call void @glfwPollEvents()
  br i1 false, label %do, label %done

done:                                             ; preds = %do, %body
  %155 = getelementptr i8, i8* %arr2, i32 0
  store i8 104, i8* %155, align 1
  %156 = getelementptr i8, i8* %arr2, i32 1
  store i8 101, i8* %156, align 1
  %157 = getelementptr i8, i8* %arr2, i32 2
  store i8 108, i8* %157, align 1
  %158 = getelementptr i8, i8* %arr2, i32 3
  store i8 108, i8* %158, align 1
  %159 = getelementptr i8, i8* %arr2, i32 4
  store i8 111, i8* %159, align 1
  %160 = getelementptr i8, i8* %arr2, i32 5
  store i8 0, i8* %160, align 1
  %161 = insertvalue { i64, i8* } { i64 6, i8* null }, i8* %arr2, 1
  %162 = extractvalue { i64, i8* } %161, 1
  %163 = insertvalue { i32, i8*, i8*, i32, i8*, i32, i32 } zeroinitializer, i8* %162, 2
  %164 = insertvalue { i32, i8*, i8*, i32, i8*, i32, i32 } %163, i32 0, 3
  %165 = getelementptr i8, i8* %arr3, i32 0
  store i8 101, i8* %165, align 1
  %166 = getelementptr i8, i8* %arr3, i32 1
  store i8 110, i8* %166, align 1
  %167 = getelementptr i8, i8* %arr3, i32 2
  store i8 103, i8* %167, align 1
  %168 = getelementptr i8, i8* %arr3, i32 3
  store i8 105, i8* %168, align 1
  %169 = getelementptr i8, i8* %arr3, i32 4
  store i8 110, i8* %169, align 1
  %170 = getelementptr i8, i8* %arr3, i32 5
  store i8 101, i8* %170, align 1
  %171 = getelementptr i8, i8* %arr3, i32 6
  store i8 0, i8* %171, align 1
  %172 = insertvalue { i64, i8* } { i64 7, i8* null }, i8* %arr3, 1
  %173 = extractvalue { i64, i8* } %172, 1
  %174 = insertvalue { i32, i8*, i8*, i32, i8*, i32, i32 } %164, i8* %173, 4
  %175 = insertvalue { i32, i8*, i8*, i32, i8*, i32, i32 } %174, i32 0, 5
  %176 = insertvalue { i32, i8*, i8*, i32, i8*, i32, i32 } %175, i32 0, 6
  store { i32, i8*, i8*, i32, i8*, i32, i32 } %176, { i32, i8*, i8*, i32, i8*, i32, i32 }* %app_info, align 8
  %177 = insertvalue { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } { i32 1, i8* null, i32 0, { i32, i8*, i8*, i32, i8*, i32, i32 }* null, i32 0, i8** null, i32 0, i8** null }, { i32, i8*, i8*, i32, i8*, i32, i32 }* %app_info, 3
  %178 = insertvalue { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } %177, i32 1, 4
  %179 = getelementptr i8*, i8** %arr4, i32 0
  %180 = getelementptr i8, i8* %arr5, i32 0
  store i8 86, i8* %180, align 1
  %181 = getelementptr i8, i8* %arr5, i32 1
  store i8 75, i8* %181, align 1
  %182 = getelementptr i8, i8* %arr5, i32 2
  store i8 95, i8* %182, align 1
  %183 = getelementptr i8, i8* %arr5, i32 3
  store i8 76, i8* %183, align 1
  %184 = getelementptr i8, i8* %arr5, i32 4
  store i8 65, i8* %184, align 1
  %185 = getelementptr i8, i8* %arr5, i32 5
  store i8 89, i8* %185, align 1
  %186 = getelementptr i8, i8* %arr5, i32 6
  store i8 69, i8* %186, align 1
  %187 = getelementptr i8, i8* %arr5, i32 7
  store i8 82, i8* %187, align 1
  %188 = getelementptr i8, i8* %arr5, i32 8
  store i8 95, i8* %188, align 1
  %189 = getelementptr i8, i8* %arr5, i32 9
  store i8 75, i8* %189, align 1
  %190 = getelementptr i8, i8* %arr5, i32 10
  store i8 72, i8* %190, align 1
  %191 = getelementptr i8, i8* %arr5, i32 11
  store i8 82, i8* %191, align 1
  %192 = getelementptr i8, i8* %arr5, i32 12
  store i8 79, i8* %192, align 1
  %193 = getelementptr i8, i8* %arr5, i32 13
  store i8 78, i8* %193, align 1
  %194 = getelementptr i8, i8* %arr5, i32 14
  store i8 79, i8* %194, align 1
  %195 = getelementptr i8, i8* %arr5, i32 15
  store i8 83, i8* %195, align 1
  %196 = getelementptr i8, i8* %arr5, i32 16
  store i8 95, i8* %196, align 1
  %197 = getelementptr i8, i8* %arr5, i32 17
  store i8 118, i8* %197, align 1
  %198 = getelementptr i8, i8* %arr5, i32 18
  store i8 97, i8* %198, align 1
  %199 = getelementptr i8, i8* %arr5, i32 19
  store i8 108, i8* %199, align 1
  %200 = getelementptr i8, i8* %arr5, i32 20
  store i8 105, i8* %200, align 1
  %201 = getelementptr i8, i8* %arr5, i32 21
  store i8 100, i8* %201, align 1
  %202 = getelementptr i8, i8* %arr5, i32 22
  store i8 97, i8* %202, align 1
  %203 = getelementptr i8, i8* %arr5, i32 23
  store i8 116, i8* %203, align 1
  %204 = getelementptr i8, i8* %arr5, i32 24
  store i8 105, i8* %204, align 1
  %205 = getelementptr i8, i8* %arr5, i32 25
  store i8 111, i8* %205, align 1
  %206 = getelementptr i8, i8* %arr5, i32 26
  store i8 110, i8* %206, align 1
  %207 = getelementptr i8, i8* %arr5, i32 27
  store i8 0, i8* %207, align 1
  %208 = insertvalue { i64, i8* } { i64 28, i8* null }, i8* %arr5, 1
  %209 = extractvalue { i64, i8* } %208, 1
  store i8* %209, i8** %179, align 8
  %210 = insertvalue { i64, i8** } { i64 1, i8** null }, i8** %arr4, 1
  %211 = extractvalue { i64, i8** } %210, 1
  %212 = insertvalue { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } %178, i8** %211, 5
  %213 = insertvalue { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } %212, i32 0, 6
  %214 = insertvalue { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } %213, i8** null, 7
  store { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** } %214, { i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** }* %createinfo, align 8
  store i8* null, i8** %instance, align 8
  %215 = call i32 @vkCreateInstance({ i32, i8*, i32, { i32, i8*, i8*, i32, i8*, i32, i32 }*, i32, i8**, i32, i8** }* %createinfo, i8* null, i8** %instance)
  store i32 1, i32* %physical_device_count, align 4
  store i8* null, i8** %physical_device, align 8
  %216 = load i8*, i8** %instance, align 8
  %217 = call i32 @vkEnumeratePhysicalDevices(i8* %216, i32* %physical_device_count, i8** %physical_device)
  store float 1.000000e+00, float* %priority, align 4
  %218 = insertvalue { i32, i32, i32, float* } { i32 2, i32 0, i32 1, float* null }, float* %priority, 3
  store { i32, i32, i32, float* } %218, { i32, i32, i32, float* }* %device_queue_createinfo, align 8
  %219 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } { i32 3, i8* null, i32 0, i32 1, { i32, i32, i32, float* }* null, i32 0, i8** null, i32 0, i8** null, i8* null }, { i32, i32, i32, float* }* %device_queue_createinfo, 4
  %220 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %219, i32 0, 5
  %221 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %220, i8** null, 6
  %222 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %221, i32 0, 7
  %223 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %222, i8** null, 8
  %224 = insertvalue { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %223, i8* null, 9
  store { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* } %224, { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* }* %device_create_info, align 8
  store i8* null, i8** %device, align 8
  %225 = load i8*, i8** %physical_device, align 8
  %226 = call i32 @vkCreateDevice(i8* %225, { i32, i8*, i32, i32, { i32, i32, i32, float* }*, i32, i8**, i32, i8**, i8* }* %device_create_info, i8* null, i8** %device)
  store i32 0, i32* %x, align 4
  %227 = load i32, i32* %x, align 4
  ret i32 %227
}

