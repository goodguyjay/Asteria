using ASN.MC.Backend.Data.Models.Requests;
using ASN.MC.Backend.Services;
using Microsoft.AspNetCore.Mvc;

namespace ASN.MC.Backend.Controllers;

[ApiController]
[Route("api/[controller]")]
public sealed class CommandController(ICommandService commandService) : ControllerBase
{
    [HttpPost]
    public async Task<IActionResult> SendCommand([FromBody] CommandRequest request)
    {
        if (string.IsNullOrEmpty(request.Command))
            return BadRequest("Command cannot be empty.");

        var result = await commandService.SendCommandAsync(request.Command);

        return Ok(new { message = "Command sent", response = result });
    }
}