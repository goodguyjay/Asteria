#pragma once

#include <Core/Task.hpp>
#include <string>

namespace ASNCommon
{
template <typename T> std::string ToString(const T &value)
{
    static_assert(sizeof(T) == -1, "ToString is not implemented for this type");
    return "";
}

// -- Specializations for ASNCore types --
template <> inline std::string ToString<ASNCore::TaskPriority>(const ASNCore::TaskPriority &value)
{
    using enum ASNCore::TaskPriority;
    switch (value)
    {
    case Low:
        return "Low";

    case Normal:
        return "Normal";

    case High:
        return "High";

    case Critical:
        return "Critical";

    default:
        return "Unknown";
    }
}

template <> inline std::string ToString<ASNCore::TaskStatus>(const ASNCore::TaskStatus &value)
{
    using enum ASNCore::TaskStatus;
    switch (value)
    {
    case Pending:
        return "Pending";
    case Running:
        return "Running";
    case Completed:
        return "Completed";
    case Failed:
        return "Failed";
    default:
        return "Unknown";
    }
}
} // namespace ASNCommon