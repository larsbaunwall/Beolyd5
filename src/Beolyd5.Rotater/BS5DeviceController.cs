namespace Beolyd5.Rotater;

public class BS5DeviceController : AbstractHidClient
{
    protected override int ProductId { get; } = 0x1112;
    protected override int VendorId { get; } = 0x0cd4;
    protected override void OnHidRead(byte[] readBuffer)
    {
        Console.WriteLine(BitConverter.ToString(readBuffer));
    }

    protected override void OnConnect()
    {
        Console.WriteLine("BS5 found");
        base.OnConnect();
        Console.WriteLine("BS5 connected");
    }
}