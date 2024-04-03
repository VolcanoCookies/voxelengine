use std::sync::Arc;

use glam::{DVec3, Vec3};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyImageToBufferInfo,
    },
    device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags},
    format::ClearColorValue,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    sync::{self, GpuFuture},
    VulkanLibrary,
};

fn main() {
    let start = std::time::Instant::now();

    let library = VulkanLibrary::new().expect("Vulkan not installed");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("Instance creation failed");

    let mut physical_devices = instance
        .enumerate_physical_devices()
        .expect("Physical device enumeration failed");
    let physical_device = physical_devices.next().expect("No devices found");

    println!(
        "Using graphics device: {:?}",
        physical_device.properties().device_name
    );

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_index, info)| info.queue_flags.contains(QueueFlags::GRAPHICS))
        .expect("No graphics queue family found");

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index: queue_family_index as u32,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Device creation failed");

    let queue = queues.next().expect("No queues found");

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let mut data = Vec::new();
    data.push(Sphere {
        center: DVec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        color: Vec3::new(0.9, 0.2, 0.1),
    });
    data.push(Sphere {
        center: DVec3::new(1.0, 2.0, -2.0),
        radius: 0.5,
        color: Vec3::new(0.8, 0.8, 0.8),
    });
    data.push(Sphere {
        center: DVec3::new(0.3, -0.75, -1.0),
        radius: 1.0,
        color: Vec3::new(0.1, 0.2, 0.9),
    });
    data.push(Sphere {
        center: DVec3::new(1.5, 0.0, -0.6),
        radius: 0.5,
        color: Vec3::new(0.8, 0.8, 0.8),
    });
    data.push(Sphere {
        center: DVec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        color: Vec3::new(0.2, 0.8, 0.0),
    });

    let buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data,
    )
    .expect("Buffer creation failed");

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index as u32,
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Command buffer creation failed");

  vulkano_shaders::shader!()

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: vulkano::format::Format::R8G8B8A8_UNORM,
            extent: [1920, 1080, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .expect("Image creation failed");

    let image_view = ImageView::new_default(image.clone());
    let layout = compute

    builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .expect("Clear color image failed");

    let image_dest = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..1920 * 1080 * 4).map(|_| 0u8),
    )
    .unwrap();

    builder
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            image_dest.clone(),
        ))
        .expect("Copy image to buffer failed");

    let command_buffer = builder.build().expect("Command buffer build failed");

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .expect("Command buffer execution failed")
        .then_signal_fence_and_flush()
        .expect("Fence and flush failed");

    future.wait(None).expect("Future wait failed");

    let buffer_content = image_dest.read().expect("Read buffer failed");
    let image = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(1920, 1080, &buffer_content[..])
        .unwrap();

    image.save("output.png").expect("Image save failed");

    println!("Elapsed: {:?}", start.elapsed());
}

#[derive(BufferContents)]
#[repr(C)]
struct Sphere {
    center: DVec3,
    radius: f64,
    color: Vec3,
}

