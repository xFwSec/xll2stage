# XLL 2 Stage Dropper
This repo contains a baseline for an XLL 2 stage dropper. We have the first stored in malicious, which when compiled uses a sliver stager to execute after breaking the parent-child of excel by creating a sacrificial Excel process.
The other is the dropper, which is the main code. This will download a file from a web server and serve it when opening to allow end users to think they've just opened a normal excel file.

Work needs doing on these, mainly adding in CRT independence, and a lot of evasive measures, but a fun play at least for now.
