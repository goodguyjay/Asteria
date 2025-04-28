#pragma once

#include <atomic>
#include <cstdint>
#include <string>
#include <vector>

namespace ASNCore
{
enum class TaskPriority
{
    Low,
    Normal,
    High,
    Critical
};

enum class TaskStatus
{
    Pending,
    Running,
    Completed,
    Failed
};

struct Task
{
    uint64_t id;
    std::vector<std::string> commands;
    TaskPriority priority;
    std::atomic<TaskStatus> status;
};
} // namespace ASNCore