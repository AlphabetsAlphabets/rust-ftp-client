> Source: https://en.wikipedia.org/wiki/File_Transfer_Protocol

# Data transfer
There are two modes but the important one is `PASV`. 
1. When the server receives a `PASV` command it gives back its IP and port that the client can connect to.
2. It can deal with NAT and convert private IP to public IP.

> I'm assuming that ports also have a private version like IPs.

## Types of data being transfered
1. ASCII (TYPE A) - Used for text.
    - ASCII has a few more sub types.
        1. TYPE A N - Non print characters. There will be no carriage control characters.
        2. TYPE A T - contains Telnet control characters such as CR, LF, etc.
        3. TYPE A A - Contains ASA control characters.
2. Image (TYPE I) - This one is pretty obvious.
3. Unicode (TYPE U) - No clue.

I believe these are the three most important ones out of the five listed in the article.
"mode" is also used instead of "type". So, TYPE A might be called "mode A" or "mode ascii".
