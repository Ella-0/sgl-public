; ModuleID = 'hello'
source_filename = "hello"
target triple = "x86_64-pc-linux-gnu"

%struct.VkInstanceCreateInfo = type { i32, i8*, i32, %struct.VkApplicationInfo*, i32, i8**, i32, i8** }
%struct.VkApplicationInfo = type { i32, i8*, i8*, i32, i8*, i32, i32 }
%struct.VkDeviceCreateInfo = type { i32, i8*, i32, i32, %struct.VkDeviceQueueCreateInfo*, i32, i8**, i32, i8**, i8* }
%struct.VkDeviceQueueCreateInfo = type { i32, i8*, i32, i32, i32, float* }
%struct.VkSemaphoreCreateInfo = type { i32, i8*, i32 }
%struct.VkSwapchainCreateInfoKHR = type { i32, i8*, i32, i8*, i32, i32, i32, %struct.VkExtent2D, i32, i32, i32, i32, i32*, i32, i32, i32, i32, i8* }
%struct.VkExtent2D = type { i32, i32 }

declare i32 @puts(i8*)

declare i64 @write(i32, i8*, i64)

declare i32 @vkCreateInstance(%struct.VkInstanceCreateInfo*, i8*, i8**)

declare i32 @vkEnumeratePhysicalDevices(i8*, i32*, i8**)

declare i32 @vkCreateDevice(i8*, %struct.VkDeviceCreateInfo*, i8*, i8**)

declare void @vkGetDeviceQueue(i8*, i32, i32, i8**)

declare i32 @vkCreateSemaphore(i8*, %struct.VkSemaphoreCreateInfo*, i8*, i8**)

declare i32 @vkCreateSwapchainKHR(i8*, %struct.VkSwapchainCreateInfoKHR*, i8*, i8**)

declare void @glfwInit()

declare void @glfwWindowHint(i32, i32)

declare i8* @glfwCreateWindow(i32, i32, i8*, i8*, i8*)

declare void @glfwPollEvents()

declare i1 @glfwWindowShouldClose(i8*)

declare i32 @glfwCreateWindowSurface(i8*, i8*, i8*, i8**)

declare void @printf(i8*, i8*)

define i32 @main() {
entry:
  %vert_spv = alloca { i64, i32* }, align 8
  %arr = alloca i32, i32 135, align 4
  %app_info = alloca %struct.VkApplicationInfo, align 8
  %arr1 = alloca i8, i32 6, align 1
  %arr2 = alloca i8, i32 7, align 1
  %createinfo = alloca %struct.VkInstanceCreateInfo, align 8
  %arr3 = alloca i8*, align 8
  %arr4 = alloca i8, i32 28, align 1
  %arr5 = alloca i8*, i32 2, align 8
  %arr6 = alloca i8, i32 15, align 1
  %arr7 = alloca i8, i32 23, align 1
  %instance = alloca i8*, align 8
  %physical_device_count = alloca i32, align 4
  %physical_device = alloca i8*, align 8
  %priority = alloca float, align 4
  %queue_family_index = alloca i32, align 4
  %device_queue_createinfo = alloca %struct.VkDeviceQueueCreateInfo, align 8
  %device_exts = alloca { i64, i8** }, align 8
  %arr8 = alloca i8*, align 8
  %arr9 = alloca i8, i32 17, align 1
  %device_create_info = alloca %struct.VkDeviceCreateInfo, align 8
  %device = alloca i8*, align 8
  %queue = alloca i8*, align 8
  %GLFW_NO_API = alloca i32, align 4
  %GLFW_TRUE = alloca i32, align 4
  %GLFW_VISIBLE = alloca i32, align 4
  %GLFW_CLIENT_API = alloca i32, align 4
  %window = alloca i8*, align 8
  %arr10 = alloca i8, i32 14, align 1
  %surface = alloca i8*, align 8
  %arr11 = alloca i8, i32 4, align 1
  %semaphore_createinfo = alloca %struct.VkSemaphoreCreateInfo, align 8
  %image_available_sem = alloca i8*, align 8
  %render_finished_sem = alloca i8*, align 8
  %swapchain_extent = alloca %struct.VkExtent2D, align 8
  %swapchain_create_info = alloca %struct.VkSwapchainCreateInfoKHR, align 8
  %swapchain = alloca i8*, align 8
  %x = alloca i32, align 4
  br label %body

body:                                             ; preds = %entry
  %0 = bitcast i32* %arr to [135 x i32]*
  store [135 x i32] [i32 119734787, i32 65536, i32 0, i32 27, i32 0, i32 131089, i32 1, i32 196622, i32 0, i32 0, i32 393221, i32 1, i32 1600939891, i32 1953654134, i32 1936683103, i32 0, i32 262215, i32 1, i32 11, i32 0, i32 196630, i32 2, i32 32, i32 262167, i32 3, i32 2, i32 4, i32 262176, i32 4, i32 3, i32 3, i32 262203, i32 4, i32 5, i32 3, i32 262149, i32 6, i32 1885302377, i32 29551, i32 262215, i32 6, i32 30, i32 0, i32 262167, i32 7, i32 2, i32 2, i32 262176, i32 8, i32 1, i32 7, i32 262203, i32 8, i32 9, i32 1, i32 262149, i32 10, i32 1601467759, i32 7565168, i32 262215, i32 10, i32 30, i32 0, i32 262176, i32 11, i32 3, i32 7, i32 262203, i32 11, i32 12, i32 3, i32 131091, i32 13, i32 131091, i32 14, i32 196641, i32 15, i32 14, i32 327734, i32 13, i32 16, i32 0, i32 15, i32 262205, i32 7, i32 17, i32 9, i32 327761, i32 2, i32 18, i32 17, i32 0, i32 262205, i32 7, i32 19, i32 9, i32 327761, i32 2, i32 20, i32 19, i32 1, i32 196630, i32 21, i32 32, i32 262187, i32 21, i32 22, i32 0, i32 196630, i32 23, i32 32, i32 262187, i32 23, i32 24, i32 1065353216, i32 458832, i32 3, i32 25, i32 18, i32 20, i32 22, i32 24, i32 196670, i32 9, i32 25, i32 65536, i32 262205, i32 7, i32 26, i32 9, i32 196670, i32 12, i32 26, i32 65536, i32 65592], [135 x i32]* %0, align 4
  %1 = insertvalue { i64, i32* } { i64 135, i32* null }, i32* %arr, 1
  store { i64, i32* } %1, { i64, i32* }* %vert_spv, align 8
  %2 = bitcast i8* %arr1 to [6 x i8]*
  store [6 x i8] c"hello\00", [6 x i8]* %2, align 1
  %3 = insertvalue { i64, i8* } { i64 6, i8* null }, i8* %arr1, 1
  %4 = extractvalue { i64, i8* } %3, 1
  %5 = insertvalue %struct.VkApplicationInfo zeroinitializer, i8* %4, 2
  %6 = insertvalue %struct.VkApplicationInfo %5, i32 0, 3
  %7 = bitcast i8* %arr2 to [7 x i8]*
  store [7 x i8] c"engine\00", [7 x i8]* %7, align 1
  %8 = insertvalue { i64, i8* } { i64 7, i8* null }, i8* %arr2, 1
  %9 = extractvalue { i64, i8* } %8, 1
  %10 = insertvalue %struct.VkApplicationInfo %6, i8* %9, 4
  %11 = insertvalue %struct.VkApplicationInfo %10, i32 0, 5
  %12 = insertvalue %struct.VkApplicationInfo %11, i32 0, 6
  store %struct.VkApplicationInfo %12, %struct.VkApplicationInfo* %app_info, align 8
  %13 = insertvalue %struct.VkInstanceCreateInfo { i32 1, i8* null, i32 0, %struct.VkApplicationInfo* null, i32 0, i8** null, i32 0, i8** null }, %struct.VkApplicationInfo* %app_info, 3
  %14 = insertvalue %struct.VkInstanceCreateInfo %13, i32 1, 4
  %15 = bitcast i8* %arr4 to [28 x i8]*
  store [28 x i8] c"VK_LAYER_KHRONOS_validation\00", [28 x i8]* %15, align 1
  %16 = insertvalue { i64, i8* } { i64 28, i8* null }, i8* %arr4, 1
  %17 = extractvalue { i64, i8* } %16, 1
  %18 = bitcast i8** %arr3 to [1 x i8*]*
  store [1 x i8*] zeroinitializer, [1 x i8*]* %18, align 8
  %19 = getelementptr i8*, i8** %arr3, i32 0
  store i8* %17, i8** %19, align 8
  %20 = insertvalue { i64, i8** } { i64 1, i8** null }, i8** %arr3, 1
  %21 = extractvalue { i64, i8** } %20, 1
  %22 = insertvalue %struct.VkInstanceCreateInfo %14, i8** %21, 5
  %23 = insertvalue %struct.VkInstanceCreateInfo %22, i32 2, 6
  %24 = bitcast i8* %arr6 to [15 x i8]*
  store [15 x i8] c"VK_KHR_surface\00", [15 x i8]* %24, align 1
  %25 = insertvalue { i64, i8* } { i64 15, i8* null }, i8* %arr6, 1
  %26 = extractvalue { i64, i8* } %25, 1
  %27 = bitcast i8* %arr7 to [23 x i8]*
  store [23 x i8] c"VK_KHR_wayland_surface\00", [23 x i8]* %27, align 1
  %28 = insertvalue { i64, i8* } { i64 23, i8* null }, i8* %arr7, 1
  %29 = extractvalue { i64, i8* } %28, 1
  %30 = bitcast i8** %arr5 to [2 x i8*]*
  store [2 x i8*] zeroinitializer, [2 x i8*]* %30, align 8
  %31 = getelementptr i8*, i8** %arr5, i32 0
  store i8* %26, i8** %31, align 8
  %32 = getelementptr i8*, i8** %arr5, i32 1
  store i8* %29, i8** %32, align 8
  %33 = insertvalue { i64, i8** } { i64 2, i8** null }, i8** %arr5, 1
  %34 = extractvalue { i64, i8** } %33, 1
  %35 = insertvalue %struct.VkInstanceCreateInfo %23, i8** %34, 7
  store %struct.VkInstanceCreateInfo %35, %struct.VkInstanceCreateInfo* %createinfo, align 8
  store i8* null, i8** %instance, align 8
  %36 = call i32 @vkCreateInstance(%struct.VkInstanceCreateInfo* %createinfo, i8* null, i8** %instance)
  store i32 1, i32* %physical_device_count, align 4
  store i8* null, i8** %physical_device, align 8
  %37 = load i8*, i8** %instance, align 8
  %38 = call i32 @vkEnumeratePhysicalDevices(i8* %37, i32* %physical_device_count, i8** %physical_device)
  store float 1.000000e+00, float* %priority, align 4
  store i32 0, i32* %queue_family_index, align 4
  %39 = insertvalue %struct.VkDeviceQueueCreateInfo { i32 2, i8* null, i32 0, i32 0, i32 1, float* null }, float* %priority, 5
  store %struct.VkDeviceQueueCreateInfo %39, %struct.VkDeviceQueueCreateInfo* %device_queue_createinfo, align 8
  %40 = bitcast i8* %arr9 to [17 x i8]*
  store [17 x i8] c"VK_KHR_swapchain\00", [17 x i8]* %40, align 1
  %41 = insertvalue { i64, i8* } { i64 17, i8* null }, i8* %arr9, 1
  %42 = extractvalue { i64, i8* } %41, 1
  %43 = bitcast i8** %arr8 to [1 x i8*]*
  store [1 x i8*] zeroinitializer, [1 x i8*]* %43, align 8
  %44 = getelementptr i8*, i8** %arr8, i32 0
  store i8* %42, i8** %44, align 8
  %45 = insertvalue { i64, i8** } { i64 1, i8** null }, i8** %arr8, 1
  store { i64, i8** } %45, { i64, i8** }* %device_exts, align 8
  %46 = insertvalue %struct.VkDeviceCreateInfo { i32 3, i8* null, i32 0, i32 1, %struct.VkDeviceQueueCreateInfo* null, i32 0, i8** null, i32 0, i8** null, i8* null }, %struct.VkDeviceQueueCreateInfo* %device_queue_createinfo, 4
  %47 = insertvalue %struct.VkDeviceCreateInfo %46, i32 0, 5
  %48 = insertvalue %struct.VkDeviceCreateInfo %47, i8** null, 6
  %49 = insertvalue %struct.VkDeviceCreateInfo %48, i32 1, 7
  %50 = load { i64, i8** }, { i64, i8** }* %device_exts, align 8
  %51 = extractvalue { i64, i8** } %50, 1
  %52 = insertvalue %struct.VkDeviceCreateInfo %49, i8** %51, 8
  %53 = insertvalue %struct.VkDeviceCreateInfo %52, i8* null, 9
  store %struct.VkDeviceCreateInfo %53, %struct.VkDeviceCreateInfo* %device_create_info, align 8
  store i8* null, i8** %device, align 8
  %54 = load i8*, i8** %physical_device, align 8
  %55 = call i32 @vkCreateDevice(i8* %54, %struct.VkDeviceCreateInfo* %device_create_info, i8* null, i8** %device)
  store i8* null, i8** %queue, align 8
  %56 = load i8*, i8** %device, align 8
  %57 = load i32, i32* %queue_family_index, align 4
  call void @vkGetDeviceQueue(i8* %56, i32 %57, i32 0, i8** %queue)
  store i32 0, i32* %GLFW_NO_API, align 4
  store i32 1, i32* %GLFW_TRUE, align 4
  store i32 0, i32* %GLFW_VISIBLE, align 4
  store i32 0, i32* %GLFW_CLIENT_API, align 4
  call void @glfwInit()
  %58 = load i32, i32* %GLFW_CLIENT_API, align 4
  %59 = load i32, i32* %GLFW_NO_API, align 4
  call void @glfwWindowHint(i32 %58, i32 %59)
  %60 = bitcast i8* %arr10 to [14 x i8]*
  store [14 x i8] c"Hello, World!\00", [14 x i8]* %60, align 1
  %61 = insertvalue { i64, i8* } { i64 14, i8* null }, i8* %arr10, 1
  %62 = extractvalue { i64, i8* } %61, 1
  %63 = call i8* @glfwCreateWindow(i32 640, i32 480, i8* %62, i8* null, i8* null)
  store i8* %63, i8** %window, align 8
  store i8* null, i8** %surface, align 8
  %64 = load i8*, i8** %instance, align 8
  %65 = load i8*, i8** %window, align 8
  %66 = call i32 @glfwCreateWindowSurface(i8* %64, i8* %65, i8* null, i8** %surface)
  %67 = bitcast i8* %arr11 to [4 x i8]*
  store [4 x i8] c"%p\0A\00", [4 x i8]* %67, align 1
  %68 = insertvalue { i64, i8* } { i64 4, i8* null }, i8* %arr11, 1
  %69 = extractvalue { i64, i8* } %68, 1
  %70 = load i8*, i8** %window, align 8
  call void @printf(i8* %69, i8* %70)
  store %struct.VkSemaphoreCreateInfo { i32 9, i8* null, i32 0 }, %struct.VkSemaphoreCreateInfo* %semaphore_createinfo, align 8
  store i8* null, i8** %image_available_sem, align 8
  store i8* null, i8** %render_finished_sem, align 8
  %71 = load i8*, i8** %device, align 8
  %72 = call i32 @vkCreateSemaphore(i8* %71, %struct.VkSemaphoreCreateInfo* %semaphore_createinfo, i8* null, i8** %image_available_sem)
  %73 = load i8*, i8** %device, align 8
  %74 = call i32 @vkCreateSemaphore(i8* %73, %struct.VkSemaphoreCreateInfo* %semaphore_createinfo, i8* null, i8** %render_finished_sem)
  store %struct.VkExtent2D { i32 640, i32 480 }, %struct.VkExtent2D* %swapchain_extent, align 4
  %75 = load i8*, i8** %surface, align 8
  %76 = insertvalue %struct.VkSwapchainCreateInfoKHR { i32 1000001000, i8* null, i32 0, i8* null, i32 0, i32 0, i32 0, %struct.VkExtent2D zeroinitializer, i32 0, i32 0, i32 0, i32 0, i32* null, i32 0, i32 0, i32 0, i32 0, i8* null }, i8* %75, 3
  %77 = insertvalue %struct.VkSwapchainCreateInfoKHR %76, i32 2, 4
  %78 = insertvalue %struct.VkSwapchainCreateInfoKHR %77, i32 50, 5
  %79 = insertvalue %struct.VkSwapchainCreateInfoKHR %78, i32 0, 6
  %80 = load %struct.VkExtent2D, %struct.VkExtent2D* %swapchain_extent, align 4
  %81 = insertvalue %struct.VkSwapchainCreateInfoKHR %79, %struct.VkExtent2D %80, 7
  %82 = insertvalue %struct.VkSwapchainCreateInfoKHR %81, i32 1, 8
  %83 = insertvalue %struct.VkSwapchainCreateInfoKHR %82, i32 0, 9
  %84 = insertvalue %struct.VkSwapchainCreateInfoKHR %83, i32 0, 10
  %85 = insertvalue %struct.VkSwapchainCreateInfoKHR %84, i32 0, 11
  %86 = insertvalue %struct.VkSwapchainCreateInfoKHR %85, i32* null, 12
  %87 = insertvalue %struct.VkSwapchainCreateInfoKHR %86, i32 1, 13
  %88 = insertvalue %struct.VkSwapchainCreateInfoKHR %87, i32 1, 14
  %89 = insertvalue %struct.VkSwapchainCreateInfoKHR %88, i32 2, 15
  %90 = insertvalue %struct.VkSwapchainCreateInfoKHR %89, i32 1, 16
  %91 = insertvalue %struct.VkSwapchainCreateInfoKHR %90, i8* null, 17
  store %struct.VkSwapchainCreateInfoKHR %91, %struct.VkSwapchainCreateInfoKHR* %swapchain_create_info, align 8
  store i8* null, i8** %swapchain, align 8
  %92 = load i8*, i8** %device, align 8
  %93 = call i32 @vkCreateSwapchainKHR(i8* %92, %struct.VkSwapchainCreateInfoKHR* %swapchain_create_info, i8* null, i8** %swapchain)
  %94 = load i8*, i8** %window, align 8
  %95 = call i1 @glfwWindowShouldClose(i8* %94)
  %96 = xor i1 %95, true
  br i1 %96, label %do, label %done

do:                                               ; preds = %do, %body
  call void @glfwPollEvents()
  %97 = load i8*, i8** %window, align 8
  %98 = call i1 @glfwWindowShouldClose(i8* %97)
  %99 = xor i1 %98, true
  br i1 %99, label %do, label %done

done:                                             ; preds = %do, %body
  store i32 0, i32* %x, align 4
  %100 = load i32, i32* %x, align 4
  ret i32 %100
}

