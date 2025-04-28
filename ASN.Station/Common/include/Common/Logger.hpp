#pragma once

#include <memory>
#include <spdlog/sinks/stdout_color_sinks.h>
#include <spdlog/spdlog.h>
#include <string>

namespace ASNLogger
{

class Logger
{
  public:
    static void Init()
    {
        if (!_logger)
        {
            _logger = spdlog::stdout_color_mt("console");
#ifdef NDEBUG
            _logger->set_level(spdlog::level::info); // Release build: only info+
#else
            _logger->set_level(spdlog::level::debug); // Debug build: debug level
#endif
        }
    }

    static void set_station_id(const std::string &id)
    {
        _stationId = id;
    }

    static const std::string &get_station_id()
    {
        return _stationId;
    }

    static void log(const std::string &message)
    {
        _logger->info("[{}]: {}", _stationId, message);
    }

    static void error(const std::string &message)
    {
        _logger->error("[ERROR] [{}]: {}", _stationId, message);
    }

    static void debug(const std::string &message)
    {
        _logger->debug("[DEBUG] [{}]: {}", _stationId, message);
    }

    static void info(const std::string &message)
    {
        _logger->info("[INFO] [{}]: {}", _stationId, message);
    }

  private:
    static inline std::string _stationId = "ASN-S1";
    static inline std::shared_ptr<spdlog::logger> _logger = nullptr;
};

} // namespace ASNLogger

// Macro wrappers
#define ASN_LOG(message) ::ASNLogger::Logger::log(message)
#define ASN_ERR(message) ::ASNLogger::Logger::error(message)
#define ASN_DEBUG(message) ::ASNLogger::Logger::debug(message)
#define ASN_INFO(message) ::ASNLogger::Logger::info(message)
