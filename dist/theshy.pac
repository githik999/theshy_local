function FindProxyForURL(url, host)
{
    if (isInNet(host, "192.168.1.0", "255.255.255.0"))
        return "DIRECT";
 
    return "SOCKS5 192.168.68.247:1080";
}