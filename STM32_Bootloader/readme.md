# **STM32_Bootloader**
[Rust](https://www.rust-lang.org) is showing great promise in the embedded world, and offers many benefits (like memory safety!). In this project, we will walk through developing a basic application for an STM32 microcontroller. I developed this using a [STM32F411VE-DISCO](https://www.st.com/en/evaluation-tools/32f411ediscovery.html) discovery board, but it should be easily adaptable to most STM32 devices.

`It is custom bootloader ,after reset bootloader is executed then it jumps to user application which is bootfirmware!`

## Toolchin-Setup 
We need to make sure that we have an up-to-date version of Rust installed. If you do not have Rust installed, please refer to the [Rust documentation](https://www.rust-lang.org/tools/install) for installation instructions. To avoid problems it's recommended to use the most recent version of Rust available to you.

    ~ rustup default nightly
    ~ rustup update
    ~ rustc --version
    rustc 1.60.0-nightly (777bb86bc 2022-01-20)

With Rust installed and up to date, we need to add support for our required compilation target, which differs depending on the type of microcontroller in use. Refer to the below table to determine the suitable target for your application :

|Target|Series|FPU|
|----|-----|-------|
|thumbv6m-none-eabi|Cortex-M0, Cortex-M0+|No|
|thumbv7m-none-eabi|Cortex-M3|No|
|thumbv7em-none-eabi|Cortex-M4, Cortex-M7|No|
|thumbv7em-none-eabihf|Cortex-M4F, Cortex-M7F|Yes|

Since I am using a Cortex-M4 device in this case, I choose the thumbv7em-none-eabihf instruction set for the compilation target:

    ~ rustup target add thumbv7em-none-eabihf

## Project-Setup
Generating the project using Cargo :

    ~ cargo new bootloader
    ~ cargo new boot_firmare

We will next add a configuration file to the project to instruct Cargo to compile for the appropriate target by default. In the root of the project, create the directory .cargo Create and open config.toml file and add the following :

    [target.'cfg(all(target_arch = "arm", target_os = "none"))']
    runner = "probe-run --chip stm32f411vetx"

    rustflags = [
    "-C", "linker=flip-link",
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-Tdefmt.x",
    # This is needed if your flash or ram addresses are not aligned to 0x10000 in memory.x
    "-C", "link-arg=--nmagic",
    ]

    [build]
    # (`thumbv6m-*` is compatible with all ARM Cortex-M chips but using the right target improves performance)
    target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
    # target = "thumbv7m-none-eabi"    # Cortex-M3
    # target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
    # target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)

    [alias]
    rb = "run --bin"
    rrb = "run --release --bin"

Since I am using a Cortex-M0 device in this case, so I have enabled `target = "thumbv7em-none-eabihf"`

Since different devices have varying amounts of Flash and RAM, we need to define a linker file, its values will need to be updated according to the values in the datasheet of your specific device (in the Memory Mapping section). `Note` that using this linker file on a device with more available Flash and/or RAM will render that memory unusable by the application. Create the file memory.x in the root of your project, and populate it with the following, updating the ORIGIN and LENGTH fields if required :

`linker file for bootloader`

    MEMORY
    {
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    /* TODO Adjust these memory regions to match your device memory layout */
    /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
    FLASH    (rx)  : ORIGIN = 0x08000000, LENGTH = 64K
    RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 28K 
    }

    /* This is where the call stack will be allocated. */
    /* The stack is of the full descending type. */
    /* You may want to use this variable to locate the call stack and static
    variables in different memory regions. Below is shown the default value */
    /* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */
    


 `linker file for boot_firmware`

 MEMORY
    {
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    /* TODO Adjust these memory regions to match your device memory layout */
    /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
    FLASH    (rx)  : ORIGIN = 0x08020000, LENGTH = 64K
    RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 28K 
    }

    /* This is where the call stack will be allocated. */
    /* The stack is of the full descending type. */
    /* You may want to use this variable to locate the call stack and static
    variables in different memory regions. Below is shown the default value */
    /* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

We'll update Cargo.toml, open it in a text editor and add the prerequisite crates under `[dependencies]`. If you are not using an STM32F4 series device, change the stm32f4xx-hal crate to the appropriate alternative.



## Finally writing the main source code!
    Source code! for this project will be contained within src/main.rs.

## Compilation & Flashing

At this point, you should be able to build the project. To do so, we simply run `cargo build --release` in the root project directory. Once the build has completed, you should see the newly created target/ directory in the project root.

probe-run this tool will be used for flashing and debugging the embedded device.

To install probe-run, use this command :

    ~ cargo install probe-run

To flash the binary use this command :

    ~ probe-run boot_firmware --chip stm32f411vetx

    ~ probe-run bootloader --chip stm32f411vetx

    Note - bootfirmware should be programmed first

    It should programmed from the respective release folder.


https://user-images.githubusercontent.com/97118799/154233080-41eaf917-308f-4caf-b107-97e851809371.mp4

Yashwanth Singh M ,2022
Licensed under [MIT License](LICENSE]

