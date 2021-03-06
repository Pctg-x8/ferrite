//! Vulkan Device and Queues

#![cfg_attr(not(feature = "Implements"), allow(dead_code))]

use vk::*;
use PhysicalDevice;
use std::ffi::CString;
use std::borrow::Cow;
use crate::VkHandle;
#[cfg(    feature = "Multithreaded") ] use std::sync::Arc as RefCounter;
#[cfg(not(feature = "Multithreaded"))] use std::rc::Rc as RefCounter;
#[cfg(feature = "Implements")] use VkResultHandler;
#[cfg(feature = "Implements")] use ::vkresolve::{Resolver, ResolverInterface};
#[cfg(feature = "Implements")] use crate::fnconv::FnTransmute;

/// Set of bit of queue flags
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct QueueFlags(VkQueueFlags);
impl QueueFlags
{
	/// Empty bits
	pub const EMPTY: Self = QueueFlags(0);
	/// Supports only graphics operations
	pub const GRAPHICS: Self = QueueFlags(VK_QUEUE_GRAPHICS_BIT);
	/// Supports only compute operations
	pub const COMPUTE: Self = QueueFlags(VK_QUEUE_COMPUTE_BIT);
	/// Supports only transfer operations
	pub const TRANSFER: Self = QueueFlags(VK_QUEUE_TRANSFER_BIT);
	/// Supports only sparse memory management operations
	pub const SPARSE_BINDING: Self = QueueFlags(VK_QUEUE_SPARSE_BINDING_BIT);
	/// Supports graphics operations
	pub const fn graphics(self) -> Self { QueueFlags(self.bits() | Self::GRAPHICS.0) }
	/// Supports compute operations
	pub const fn compute(self) -> Self { QueueFlags(self.0 | Self::COMPUTE.0) }
	/// Supports transfer operations
	pub const fn transfer(self) -> Self { QueueFlags(self.0 | Self::TRANSFER.0) }
	/// Supports sparse memory management operatinons
	pub const fn sparse_binding(self) -> Self { QueueFlags(self.0 | Self::SPARSE_BINDING.0) }

	pub const fn bits(self) -> VkQueueFlags { self.0 }
}
/// List of queue families
pub struct QueueFamilies(pub Vec<VkQueueFamilyProperties>);
impl QueueFamilies
{
	/// Find a queue family index containing specified bitflags
	#[allow(non_snake_case)]
	pub fn find_matching_index(&self, flags: QueueFlags) -> Option<u32>
	{
		self.0.iter().position(|q| (q.queueFlags & flags.0) != 0).map(|x| x as _)
	}
	/// Find a queue family index containing specified bitflags
	#[allow(non_snake_case)]
	pub fn find_another_matching_index(&self, flags: QueueFlags, exclude: u32) -> Option<u32>
	{
		self.0.iter().enumerate().find(|&(n, &VkQueueFamilyProperties { queueFlags, .. })| (queueFlags & flags.0) != 0 && exclude != n as u32)
			.map(|(n, _)| n as _)
	}
	/// Number of queue families
	pub fn count(&self) -> u32 { self.0.len() as _ }
	/// Number of queues in selected queue family
	pub fn queue_count(&self, family_index: u32) -> u32 { self.0[family_index as usize].queueCount }
	/// Unsigned integer count of meaningful bits in the timestamps written via `vkCmdWriteTimestamp`
	pub fn timestamp_valid_bits(&self, family_index: u32) -> u32 { self.0[family_index as usize].timestampValidBits }
	/// Minimum granularity supported for image transfer operations on the queues in selected queue family
	pub fn minimum_image_transfer_granularity(&self, family_index: u32) -> &VkExtent3D { &self.0[family_index as usize].minImageTransferGranularity }
}

struct DeviceCell(VkDevice, ::Instance);
/// Opaque handle to a device object
#[derive(Clone)]
pub struct Device(RefCounter<DeviceCell>);
#[cfg(feature = "Multithreaded")] unsafe impl Sync for Device {}
/// Opaque handle to a queue object
#[derive(Clone)]
pub struct Queue(VkQueue, Device);
/// Family Index, Queue Priorities
pub struct DeviceQueueCreateInfo(pub u32, pub Vec<f32>);

#[cfg(feature = "Implements")]
impl Drop for DeviceCell { fn drop(&mut self) { unsafe { Resolver::get().destroy_device(self.0, ::std::ptr::null()) }; } }

impl VkHandle for Device { type Handle = VkDevice; fn native_ptr(&self) -> VkDevice { self.0 .0 } }
impl VkHandle for Queue  { type Handle = VkQueue;  fn native_ptr(&self) -> VkQueue  { self.0 } }
impl ::DeviceChild for Queue { fn device(&self) -> &Device { &self.1 } }

/// Builder object for constructing a `Device`
pub struct DeviceBuilder<'p>
{
	pdev_ref: &'p PhysicalDevice, queue_infos: Vec<DeviceQueueCreateInfo>,
	layers: Vec<CString>, extensions: Vec<CString>, features: VkPhysicalDeviceFeatures
}
impl<'p> DeviceBuilder<'p>
{
	pub fn new(pdev: &'p PhysicalDevice) -> Self
	{
		DeviceBuilder { pdev_ref: pdev, queue_infos: Vec::new(), layers: Vec::new(), extensions: Vec::new(), features: VkPhysicalDeviceFeatures::default() }
	}
	pub fn add_layer(&mut self, name: &str) -> &mut Self { self.layers.push(CString::new(name).unwrap()); self }
	pub fn add_extension(&mut self, name: &str) -> &mut Self
	{
		self.extensions.push(CString::new(name).unwrap()); self
	}
	pub fn add_extension_zerotermed(&mut self, name: &str) -> &mut Self
	{
		self.extensions.push(unsafe { ::std::ffi::CStr::from_ptr(name.as_ptr() as *const _) }.to_owned());
		self
	}
	pub fn add_layers<'s, Layers: IntoIterator<Item = &'s str>>(&mut self, layers: Layers) -> &mut Self
	{
		for l in layers { self.add_layer(l); } self
	}
	pub fn add_extensions<'s, Extensions: IntoIterator<Item = &'s str>>(&mut self, extensions: Extensions) -> &mut Self
	{
		for e in extensions { self.add_extension(e); } self
	}
	pub fn add_queue(&mut self, info: DeviceQueueCreateInfo) -> &mut Self { self.queue_infos.push(info); self }
	pub fn add_queues<Queues: IntoIterator<Item = DeviceQueueCreateInfo>>(&mut self, queues: Queues) -> &mut Self
	{
		for q in queues { self.add_queue(q); } self
	}
	pub fn mod_features(&mut self) -> &mut VkPhysicalDeviceFeatures { &mut self.features }
	/// [feature = "Implements"] Create a new device instance
	/// # Failures
	/// On failure, this command returns
	///
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_INITIALIZATION_FAILED`
	/// * `VK_ERROR_EXTENSION_NOT_PRESENT`
	/// * `VK_ERROR_FEATURE_NOT_PRESENT`
	/// * `VK_ERROR_TOO_MANY_OBJECTS`
	/// * `VK_ERROR_DEVICE_LOST`
	#[cfg(feature = "Implements")]
	pub fn create(&self) -> ::Result<Device>
	{
		let qinfos = self.queue_infos.iter().map(|&DeviceQueueCreateInfo(fi, ref ps)| ::vk::VkDeviceQueueCreateInfo
		{
			queueFamilyIndex: fi, queueCount: ps.len() as _, pQueuePriorities: ps.as_ptr(), .. Default::default()
		}).collect::<Vec<_>>();
		let layers = self.layers.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
		let extensions = self.extensions.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
		let cinfo = ::vk::VkDeviceCreateInfo
		{
			queueCreateInfoCount: qinfos.len() as _, pQueueCreateInfos: qinfos.as_ptr(),
			enabledLayerCount: layers.len() as _, ppEnabledLayerNames: layers.as_ptr(),
			enabledExtensionCount: extensions.len() as _, ppEnabledExtensionNames: extensions.as_ptr(),
			pEnabledFeatures: &self.features, .. Default::default()
		};
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { Resolver::get().create_device(self.pdev_ref.native_ptr(), &cinfo, ::std::ptr::null(), &mut h) }.into_result()
			.map(|_| Device(RefCounter::new(DeviceCell(h, self.pdev_ref.parent().clone()))))
	}
}
/// Tweaking features
impl<'p> DeviceBuilder<'p>
{
	pub fn enable_fill_mode_nonsolid(&mut self) -> &mut Self
	{
		self.features.fillModeNonSolid = true as _; self
	}
	pub fn enable_sample_rate_shading(&mut self) -> &mut Self
	{
		self.features.sampleRateShading = true as _; self
	}
	pub fn enable_geometry_shader(&mut self) -> &mut Self
	{
		self.features.geometryShader = true as _; self
	}
	pub fn enable_tessellation_shader(&mut self) -> &mut Self
	{
		self.features.tessellationShader = true as _; self
	}
	pub fn enable_vertex_pipeline_stores_and_atomics(&mut self) -> &mut Self
	{
		self.features.vertexPipelineStoresAndAtomics = true as _; self
	}
}
impl Device
{
	pub(crate) fn instance(&self) -> &::Instance { &self.0 .1 }
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Device
{
	/// Return a function pointer for a command
	/// # Failures
	/// If function is not provided by instance or `name` is empty, returns `None`
	pub fn extra_procedure<F: ::fnconv::FnTransmute>(&self, name: &str) -> Option<F>
	{
		if name.is_empty() { return None; }

		unsafe
		{
			Resolver::get().get_device_proc_addr(self.native_ptr(), CString::new(name).unwrap().as_ptr())
				.map(|f| FnTransmute::from_fn(f))
		}
	}
	/// Get a queue handle from a device
	pub fn queue(&self, family_index: u32, queue_index: u32) -> Queue
	{
		let mut h = unsafe { ::std::mem::zeroed() };
		unsafe { Resolver::get().get_device_queue(self.native_ptr(), family_index, queue_index, &mut h) }
		Queue(h, self.clone())
	}
	/// Invalidate `MappedMemoryRange`s
	/// Invalidating the memory range allows that device writes to the memory ranges
	/// which have been made visible to the `VK_ACCESS_HOST_WRITE_BIT` and `VK_ACCESS_HOST_READ_BIT`
	/// are made visible to the host
	/// # Safety
	/// Memory object in `ranges` must be currently host mapped
	pub unsafe fn invalidate_memory_range(&self, ranges: &[VkMappedMemoryRange]) -> ::Result<()>
	{
		Resolver::get()
			.invalidate_mapped_memory_ranges(self.native_ptr(), ranges.len() as _, ranges.as_ptr()).into_result()
	}
	/// Update the contents of a descriptor set object
	pub fn update_descriptor_sets(&self, write: &[::DescriptorSetWriteInfo], copy: &[::DescriptorSetCopyInfo])
	{
		// save flatten results
		let wt = write.iter().map(|x|
		{
			let (ty, count, imgs, bufs, bufviews) = x.3.decomposite();
			(x.0, x.1, x.2, ty, count,
				imgs.iter().map(|&(s, v, l)| VkDescriptorImageInfo
				{
					sampler: s.unwrap_or(VK_NULL_HANDLE as _), imageView: v, imageLayout: l as _
				}).collect::<Vec<_>>(),
				bufs.iter().map(|&(b, ref r)| VkDescriptorBufferInfo
				{
					buffer: b, offset: r.start as _, range: (r.end - r.start) as _
				}).collect::<Vec<_>>(), bufviews)
		}).collect::<Vec<_>>();
		let w = wt.iter().map(|&(set, binding, array, dty, count, ref iv, ref bv, ref bvv)| VkWriteDescriptorSet
		{
			dstSet: set, dstBinding: binding, dstArrayElement: array, descriptorType: dty as _, descriptorCount: count,
			pImageInfo: iv.as_ptr(), pBufferInfo: bv.as_ptr(), pTexelBufferView: bvv.as_ptr(), .. Default::default()
		}).collect::<Vec<_>>();
		let c = copy.iter().map(|x| VkCopyDescriptorSet
		{
			srcSet: x.src.0, srcBinding: x.src.1, srcArrayElement: x.src.2,
			dstSet: x.dst.0, dstBinding: x.dst.1, dstArrayElement: x.dst.2, descriptorCount: x.count, .. Default::default()
		}).collect::<Vec<_>>();
		unsafe { Resolver::get().update_descriptor_sets(self.native_ptr(), w.len() as _, w.as_ptr(), c.len() as _, c.as_ptr()) };
	}
}

/// [feature = "Implements"] Supports blocking wait operation
#[cfg(feature = "Implements")]
pub trait Waitable
{
	/// Wait for a object to become idle
	fn wait(&self) -> ::Result<()>;
}
#[cfg(feature = "Implements")]
impl Waitable for Device { fn wait(&self) -> ::Result<()> { unsafe { Resolver::get().device_wait_idle(self.native_ptr()) }.into_result() } }
#[cfg(feature = "Implements")]
impl Waitable for Queue { fn wait(&self) -> ::Result<()> { unsafe { Resolver::get().queue_wait_idle(self.0) }.into_result() } }

/// Sparse Binding operation batch
pub struct SparseBindingOpBatch<'s>
{
	/// An array of semaphores upon which to wait on before the sparse binding operations
	/// for this batch begin execution
	pub wait_semaphores: Cow<'s, [&'s ::Semaphore]>,
	/// An array of `VkSparseBufferMemoryBindInfo` structures
	pub buffer_binds: Cow<'s, [VkSparseBufferMemoryBindInfo]>,
	/// An array of `VkSparseImageOpaqueMemoryBindInfo` structures
	pub image_opaque_binds: Cow<'s, [VkSparseImageOpaqueMemoryBindInfo]>,
	/// An array of `VkSparseImageMemoryBindInfo` structures
	pub image_binds: Cow<'s, [VkSparseImageMemoryBindInfo]>,
	/// An array of semaphores which will be signaled when the sparse binding
	/// operations for this batch have completed execution
	pub signal_semaphores: Cow<'s, [&'s ::Semaphore]>
}
impl<'s> Default for SparseBindingOpBatch<'s>
{
	fn default() -> Self
	{
		SparseBindingOpBatch
		{
			wait_semaphores: Cow::Owned(Vec::new()),
			buffer_binds: Cow::Owned(Vec::new()), image_opaque_binds: Cow::Owned(Vec::new()), image_binds: Cow::Owned(Vec::new()),
			signal_semaphores: Cow::Owned(Vec::new())
		}
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Queue
{
	/// Bind device memory to a sparse resource object
	/// # Failure
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	pub fn bind_sparse(&self, batches: &[SparseBindingOpBatch], fence: Option<&::Fence>) -> ::Result<()>
	{
		let sem_ptrs = batches.iter().map(|x| (x.wait_semaphores.iter().map(|x| x.0).collect(), x.signal_semaphores.iter().map(|x| x.0).collect()));
		let batches = batches.iter().zip(sem_ptrs).map(|(x, (ws, ss)): (&SparseBindingOpBatch, (Vec<_>, Vec<_>))| VkBindSparseInfo
		{
			waitSemaphoreCount: ws.len() as _, pWaitSemaphores: ws.as_ptr(),
			bufferBindCount: x.buffer_binds.len() as _, pBufferBinds: x.buffer_binds.as_ptr(),
			imageOpaqueBindCount: x.image_opaque_binds.len() as _, pImageOpaqueBinds: x.image_opaque_binds.as_ptr(),
			imageBindCount: x.image_binds.len() as _, pImageBinds: x.image_binds.as_ptr(),
			signalSemaphoreCount: ss.len() as _, pSignalSemaphores: ss.as_ptr(),
			.. Default::default()
		}).collect::<Vec<_>>();
		unsafe { Resolver::get().queue_bind_sparse(self.0, batches.len() as _, batches.as_ptr(), fence.map(|x| x.0).unwrap_or(VK_NULL_HANDLE as _)) }
			.into_result()
	}
}

/// Semaphore/Command submission operation batch
pub struct SubmissionBatch<'d>
{
	pub wait_semaphores: Cow<'d, [(&'d ::Semaphore, ::PipelineStageFlags)]>,
	pub command_buffers: Cow<'d, [::CommandBuffer]>,
	pub signal_semaphores: Cow<'d, [&'d ::Semaphore]>
}
impl<'d> Default for SubmissionBatch<'d>
{
	fn default() -> Self
	{
		SubmissionBatch
		{
			wait_semaphores: Cow::Borrowed(&[]), command_buffers: Cow::Borrowed(&[]),
			signal_semaphores: Cow::Borrowed(&[])
		}
	}
}
/// Following methods are enabled with [feature = "Implements"]
#[cfg(feature = "Implements")]
impl Queue
{
	/// Submits a sequence of semaphores or command buffers to a queue
	/// # Failure
	/// On failure, this command returns
	/// 
	/// * `VK_ERROR_OUT_OF_HOST_MEMORY`
	/// * `VK_ERROR_OUT_OF_DEVICE_MEMORY`
	/// * `VK_ERROR_DEVICE_LOST`
	pub fn submit(&self, batches: &[SubmissionBatch], fence: Option<&::Fence>) -> ::Result<()>
	{
		let sem_ptrs: Vec<((Vec<_>, Vec<_>), Vec<_>, Vec<_>)> = batches.iter().map(|x| (
			x.wait_semaphores.iter().map(|&(ref x, p)| (x.native_ptr(), p.0)).unzip(),
			x.command_buffers.iter().map(|x| x.native_ptr()).collect(),
			x.signal_semaphores.iter().map(|x| x.native_ptr()).collect()
		)).collect();
		let batches: Vec<_> = sem_ptrs.iter().map(|&(ref ws, ref cbs, ref ss)| VkSubmitInfo
		{
			waitSemaphoreCount: ws.0.len() as _, pWaitSemaphores: ws.0.as_ptr(), pWaitDstStageMask: ws.1.as_ptr(),
			commandBufferCount: cbs.len() as _, pCommandBuffers: cbs.as_ptr(),
			signalSemaphoreCount: ss.len() as _, pSignalSemaphores: ss.as_ptr(),
			.. Default::default()
		}).collect();
		unsafe { Resolver::get().queue_submit(self.native_ptr(), batches.len() as _, batches.as_ptr(), fence.map(VkHandle::native_ptr).unwrap_or(VK_NULL_HANDLE as _)) }
			.into_result()
	}
}
