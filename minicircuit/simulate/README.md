# MiniCircuit Simulator

This simulator allows you to test your MiniCircuit driver code without a physical device. It creates a virtual COM port pair and simulates a MiniCircuit device on one end, allowing your application to connect to the other end.

## Getting Started

1. Navigate to the `minicircuit/simulate` directory
2. Run `cargo run` to start the simulator
3. The simulator will:
   - Create a virtual COM port pair
   - Print the port names to connect to
   - Start listening for commands on the device port
   - Periodically print the command log

## Setting up Virtual COM Ports

### Windows Users

1. Install com0com from http://com0com.sourceforge.net/ (use the x64 version for 64-bit systems)
2. Run the com0com setup tool as administrator (typically located in `C:\Program Files\com0com\`)
3. First, identify which COM ports your physical MiniCircuit device uses:
   - Open Device Manager (right-click Start menu → Device Manager)
   - Expand "Ports (COM & LPT)"
   - Note the COM port number used by your MiniCircuit device

4. Create virtual COM port pairs:
   - In the com0com setup tool, click "Add Pair" to create a new port pair
   - Check "use Ports class" for both ports
   - Leave all other options unchecked
   - Note the COM port numbers assigned (e.g., COM3 and COM4)
   - If these conflict with your physical MiniCircuit device, continue adding more pairs
   - com0com will increment the port numbers with each new pair

5. Keep adding pairs until you get one with non-conflicting port numbers
   - For example, if your MiniCircuit uses COM3, keep adding pairs until you get COM5/COM6 or higher
   - Then delete any unnecessary pairs by selecting them and clicking "Remove Pair"
   - Keep only the non-conflicting pair that you'll use with the simulator

6. Verify the final ports in Device Manager:
   - Open Device Manager and expand "Ports (COM & LPT)"
   - Confirm your chosen virtual COM ports are listed
   - Make note of these port numbers for use with the simulator

**Important**: Using conflicting COM ports can cause connection issues with your physical device, so always ensure your virtual ports use different numbers.

### Linux/Mac Users

Install socat:
- Ubuntu/Debian: `sudo apt-get install socat`
- macOS: `brew install socat`

The simulator will automatically create virtual ports using socat.

## Connecting Your Application

In your application, connect to the client port printed by the simulator:

```rust
let mut target_properties = TargetProperties::default();
target_properties.port = Some(String::from("COM4")); // Use the client port name printed by the simulator
let mut controller = MiniCircuitDriver::new(target_properties);

match controller.port_connect() { // Use port_connect() instead of connect() to use the specific port
    Ok(channels) => {
        let channel_tx = channels.0;
        let log_rx = Arc::new(Mutex::new(channels.1));
        // ...
    }
    Err(e) => {
        // ...
    }
}
```

## Important Notes

1. **Windows Users**: You need to install com0com (http://com0com.sourceforge.net/) to create virtual COM port pairs. Follow these instructions for more details (https://www.youtube.com/watch?v=Z8jFWQYxSNc).
   
   If you have errors like the COM port number showing up as COM# instead of a real port "COM4", also install the signed version of com0com from (https://pete.akeo.ie/2011/07/com0com-signed-drivers.html).  Quick Reference, when downloading the signed com0com, extract the zip file, and right click on com0com.inf and click "install".
   
   If it still doesn't work after this, follow these instructions (https://www.youtube.com/watch?v=1UEK4RDy1Y8)

2. **Linux/Mac Users**: You need to install socat (`sudo apt-get install socat` or `brew install socat`).

3. **Using port_connect()**: Make sure to use `controller.port_connect()` instead of `controller.connect()` since you're connecting to a specific port rather than auto-detecting.

4. **Permissions**: On Linux/Mac, you might need appropriate permissions to access the serial ports.

## Supported Commands

The simulator currently supports the following commands:

- `$FCG` - Get frequency
- `$FCS` - Set frequency
- `$ECS` - Set RF output
- `$ECG` - Get RF output
- `$PCG` - Get phase
- `$PCS` - Set phase
- `$IDN` - Get identity
- `$TCG` - Get ISC temperature
- `$RTG` - Get uptime
- `$ST` - Get status
- `$RST` - Reset system

You can extend the simulator by adding more command handlers in the `simulator.rs` file.

## Troubleshooting

If you encounter issues:

1. Make sure the virtual COM port software is installed and running
2. Check that the ports are not in use by another application
3. Verify you're using the correct port name in your application
4. On Linux/Mac, ensure you have the necessary permissions to access the ports

