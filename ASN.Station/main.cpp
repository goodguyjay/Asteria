#include "Logger.hpp"

int main()
{
    asn::Logger::init();
    ASN_INFO("Asteria Station Logger online.");
    ASN_DEBUG("Debugging enabled");
    ASN_ERR("This is an error");
    ASN_LOG("Normal log");

    return 0;
}