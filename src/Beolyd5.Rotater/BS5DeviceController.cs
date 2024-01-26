// ReSharper disable InconsistentNaming
namespace Beolyd5.Rotater;

public class BS5DeviceController : AbstractHidClient
{
    public enum EventTypes
    {
        ButtonLeft = 0x40,
        ButtonRight = 0x80,
        ButtonGo = 0x50,
        ButtonBack = 0x70,
        Wheel1,
        Wheel2,
        Wheel3,
        
    }
    
    protected override int ProductId { get; } = 0x1112;
    protected override int VendorId { get; } = 0x0cd4;

    private byte[] _lastKnownRead = new byte[6];
    
    public EventHandler<BS5DeviceEventArgs>? OnDeviceEvent;
    
    protected override void OnHidRead(byte[] readBuffer)
    {
        EventSynchronizationContext.Post(_ =>
        {
            if (readBuffer.SequenceEqual(_lastKnownRead)) return;
            
            // Detect button presses
            foreach (EventTypes button in Enum.GetValues(typeof(EventTypes)))
            {
                // Check each byte in the buffer
                foreach (var b in readBuffer)
                {
                    // If the bitwise AND is not zero, the button is pressed
                    if ((b & (byte)button) != 0)
                    {
                        // Raise an event for the button
                        OnDeviceEvent?.Invoke(this, new BS5DeviceEventArgs(readBuffer, _lastKnownRead, button));
                    }
                }
            }
        }, null);
        
        _lastKnownRead = readBuffer;
    }
    
    private void DetectEvent(byte[] readBuffer)
    {
        
    }
}

public class BS5DeviceEventArgs(byte[] readBuffer, byte[] lastKnownBuffer, BS5DeviceController.EventTypes buttonPressed) : EventArgs
{
    public byte[] ReadBuffer { get; private set; } = readBuffer;
    public byte[] LastKnownBuffer { get; private set; } = lastKnownBuffer;
    public BS5DeviceController.EventTypes ButtonPressed { get; } = buttonPressed;
}