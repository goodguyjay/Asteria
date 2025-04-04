namespace ASN.MC.Backend.Services;

public interface ICommandService
{
    Task<string> SendCommandAsync(string command);
}