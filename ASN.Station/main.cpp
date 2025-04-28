#include <Common/Logger.hpp>
#include <Common/ToString.hpp>
#include <Core/Task.hpp>
#include <iostream>

int main()
{
    ASNLogger::Logger::Init();
    ASN_INFO("Asteria Station Logger online.");
    ASN_DEBUG("Debugging enabled");
    ASN_ERR("This is an error");
    ASN_LOG("Normal log");

    static uint64_t idCounter = 1;

    const ASNCore::Task task{idCounter++,
                             {"crit_init_solar_panels()", "low_picture(direction=\"NE\")"},
                             ASNCore::TaskPriority::High,
                             ASNCore::TaskStatus::Pending};

    ASN_INFO("Created Task:");
    std::cout << "TASK ID: " << task.id << '\n';
    std::cout << "Priority: " << ASNCommon::ToString(task.priority) << '\n';
    std::cout << "Status: " << ASNCommon::ToString(task.status.load()) << '\n';

    std::cout << "Commands:" << '\n';

    for (const auto &cmd : task.commands)
    {
        std::cout << " - " << cmd << '\n';
    }

    return 0;
}