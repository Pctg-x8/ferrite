initSidebarItems({"enum":[["AddressingMode","Specify behavior of sampling with texture coordinates outside an image"],["BasePipeline","Which is pipeline state to derive from"],["BorderColor","Specify border color used for texture lookups"],["BufferSparseBinding","Bitset specifying additional parameters of a buffer"],["ClearValue","The enum representation of `VkClearValue`"],["CompareOp","Stencil comparison function"],["ComponentSwizzle","Specify how a component is swizzled"],["CompositeAlpha",""],["DebugReportObjectType","The type of an object passed to the `VkDebugMarkerObjectNameInfoEXT` and `VkDebugMarkerObjectTagInfoEXT` commands"],["DescriptorType","Specified the type of a descriptor in a descriptor set"],["DescriptorUpdateInfo","Structure specifying the parameters of a descriptor set write/copy operations For Sampler, CombinedImageSampler, SampledImage, StorageImage and InputAttachment: Vec of tuple(ref to Sampler(optional), ref to ImageView, ImageLayout) For UniformBuffer, StorageBuffer, UniformBufferDynamic and StorageBufferDynamic: Vec of tuple(ref to Buffer, range of bytes) For UniformTexelBuffer and StorageTexelBuffer: Vec of ref to BufferView"],["DisplayPlaneAlpha","Alpha blending type"],["DynamicArrayState","Whether the state(type of array) is dynamic or static"],["ElementType","Containing component element in format"],["FilterMode","Specify filter used for texture lookups"],["FormatComponents","Containing Components in Format(Order is not considered)"],["ImageCell",""],["ImageLayout","Layouts of image and image subresources"],["IndexType","Type of index buffer indices"],["LogicOp","Framebuffer logical operations"],["MipmapFilterMode","Specify mipmap mode used for texture lookups"],["OcclusionQuery","Enabling or disabling the occlusion query"],["PresentMode","Presentation mode supported for a surface"],["QueryType","Specify the type of queries managed by a query pool"],["StencilFaceMask","Bitmask specifying sets of stencil state for which to update the compare mask"],["StencilOp","Stencil action function"],["SurfaceTransform",""],["SwitchOrDynamicState","Disabled, Specified in the command buffer or Specified in the pipeline state"]],"macro":[["VK_MAKE_VERSION",""],["VK_VERSION",""]],"mod":[["traits","All of traits"],["vk","Vulkan API Definitions 1.0.59.0"]],"struct":[["AspectMask","Bitmask specifying which aspects of an image are included in a view"],["Buffer","Opaque handle to a buffer object(constructed via `BufferDesc`)"],["BufferDesc","Builder structure specifying the parameters of a newly created buffer object"],["BufferUsage","Bitmask specifying allowed usage of a buffer"],["BufferView","Opaque handle to a buffer view object"],["CmdRecord","The recording state of commandbuffers"],["CommandBuffer","Opaque handle to a command buffer object"],["CommandPool","Opaque handle to a command pool object"],["ComponentMapping","Structure specifying a color component mapping"],["DSLBindings","Structure specifying a descriptor set layout binding"],["DebugReportCallback","Opaque object to a debug report callback object"],["DebugReportCallbackBuilder",""],["DescriptorPool","Opaque handle to a descriptor pool object"],["DescriptorPoolSize","Structure specifying descriptor pool size"],["DescriptorSetCopyInfo","Structure specifying a copy descriptor set operation"],["DescriptorSetLayout","Opaque handle to a descriptor set layout object"],["DescriptorSetWriteInfo","Structure specifying the parameters of a descriptor set write operation Element order: DescriptorSet, Binding, ArrayIndex, Description"],["Device","Opaque handle to a device object"],["DeviceBuilder","Builder object for constructing a `Device`"],["DeviceMemory","Opaque handle to a device memory object"],["DeviceQueueCreateInfo","Family Index, Queue Priorities"],["DynamicDataCell","Untyped data cell"],["Event","Opaque handle to a event object"],["Extent1D",""],["Extent2D",""],["Extent3D",""],["Extent4D",""],["Fence","Opaque handle to a fence object"],["FormatQuery","Format Selection Query"],["FormatTraits","For testing format traits"],["Framebuffer","Opaque handle to a framebuffer object"],["GraphicsPipelineBuilder","Builder struct to construct a `Pipeline` for graphics operations"],["Image","Opaque handle to a image object(constructed via `ImageDesc`)"],["ImageDesc","Builder structure specifying the parameters of a newly created image object"],["ImageFlags","Bitmask specifying additional parameters of an image"],["ImageSubresourceRange","Structure specifying a image subresource range"],["ImageUsage","Bitmask specifying intended usage of an image"],["ImageView","Opaque handle to a image view object"],["Instance","Opaque handle to a instance object"],["InstanceBuilder","Builder object for constructing a `Instance`"],["MappedMemoryRange","Specifies the block of mapped memory in a `DeviceMemory`"],["MemoryProperties","Device memory properties"],["MemoryPropertyFlags","Bitmask specifying properties for a memory type"],["Offset1D",""],["Offset2D",""],["Offset3D",""],["Offset4D",""],["PhysicalDevice","Opaque handle to a physical device object"],["Pipeline","Opaque handle to a pipeline object"],["PipelineCache","Opaque handle to a pipeline cache object"],["PipelineLayout","Opaque handle to a pipeline layout object"],["PipelineShader","Builder struct to construct a shader stage in a `Pipeline`"],["PipelineStageFlags","Bitmask specifying pipeline stages"],["QueryPipelineStatisticFlags","Bitmask specifying queried pipeline statistics"],["QueryPool","Opaque handle to a query pool object"],["QueryResultFlags","Bitmask specifying how and when query results are returned"],["Queue","Opaque handle to a queue object"],["QueueFamilies","List of queue families"],["QueueFlags","Set of bit of queue flags"],["RenderPass","Opaque handle to a render pass object"],["RenderPassBuilder","Builder structure to construct the `RenderPass`"],["Sampler","Opaque handle to a sampler object"],["SamplerBuilder","Builder object for constructing the sampler object"],["Semaphore","Opaque handle to a semaphore object"],["ShaderModule","Opaque handle to a shader module object"],["ShaderStage","Bitmask specifying a pipeline stage"],["SparseBindingOpBatch","Sparse Binding operation batch"],["SubmissionBatch","Semaphore/Command submission operation batch"],["SubpassDescription","Builder structure to construct the `VkSubpassDescription`"],["Surface","Opaque handle to a surface object"],["Swapchain","Opaque handle to a swapchain object"],["SwapchainBuilder","Builder object to construct a `Swapchain`"],["VkResultBox","Boxed version of `VkResult`"]],"trait":[["ClearColorValue","The trait representation of `VkClearColorValue`"],["DeviceChild","Child of a device object"],["ImageSize","Image Dimension by corresponding extent type"],["MemoryBound","[feature = \"FeImplements\"] Common operations for memory bound objects"],["Status",""],["VkHandle","Wrapping a Vulkan Dispatchable/Nondispatchable Handler"],["VkResultHandler",""],["Waitable","[feature = \"FeImplements\"] Supports blocking wait operation"]],"type":[["Result",""]]});