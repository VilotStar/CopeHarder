byte1 = 192
byte2 = 168
byte3 = 0
byte4 = 1

ip = (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | (byte4 << 0)
ip = ip + 255
print((ip & 0xff000000) >> 24, (ip & 0x00ff0000) >> 16, (ip & 0x0000ff00) >> 8, ip & 0x000000ff)

# private static string GetNextIpAddress(string ipAddress, uint increment)
#     {
#         byte[] addressBytes = IPAddress.Parse(ipAddress).GetAddressBytes().Reverse().ToArray();
#         uint ipAsUint = BitConverter.ToUInt32(addressBytes, 0);
#         var nextAddress = BitConverter.GetBytes(ipAsUint + increment);
#         return String.Join(".", nextAddress.Reverse());
#     }