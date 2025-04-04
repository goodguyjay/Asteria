using System.Net.Sockets;
using System.Text;

namespace ASN.MC.Backend.Services;

public sealed class CommandService(ILogger<CommandService> logger) : ICommandService
{
    // ReSharper disable once InconsistentNaming
    private const string ASNIp = "127.0.0.1";

    // ReSharper disable once InconsistentNaming
    private const int ASNPort = 5001;

    public async Task<string> SendCommandAsync(string command)
    {
        try
        {
            logger.LogInformation("[Mission Control] Sending command: {command}", command);

            using var client = new TcpClient();
            await client.ConnectAsync(ASNIp, ASNPort);

            await using var stream = client.GetStream();
            var data = Encoding.UTF8.GetBytes(command);

            await stream.WriteAsync(data, 0, data.Length);
            logger.LogInformation("[Mission Control] Command sent: {command}", command);

            var buffer = new byte[1024];
            var bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);
            var response = Encoding.UTF8.GetString(buffer, 0, bytesRead);

            logger.LogInformation("[Mission Control] Command response: {response}", response);
            return response;
        }
        catch (Exception exception)
        {
            logger.LogError(
                "[Mission Control] An error occurred while sending command: {exception}",
                exception.Message
            );

            return "Error: Failed to send command.";
        }
    }
}
