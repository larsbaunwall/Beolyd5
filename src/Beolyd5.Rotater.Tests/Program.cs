// See https://aka.ms/new-console-template for more information

using Beolyd5.Rotater;

HidSharp.Utility.HidSharpDiagnostics.EnableTracing = true;

Console.WriteLine("Connecting again ...");
var device = new BS5DeviceController();


Console.ReadKey();