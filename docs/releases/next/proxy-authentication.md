### Improved

- The network proxy takes a username and password, for a proxy that asks for them: requests then identify themselves with Basic auth. Like the proxy's address, the credentials stay on this computer and are never written to a settings file.

### Fixed

- Plugins, the theme catalog, downloads, web search and update checks reach the internet again from an intranet where only a proxy can: a SOCKS5 proxy now really connects (names are resolved at the proxy, which this computer's own DNS cannot do), and with no proxy set in Textus the app follows the computer's own proxy, as other apps do.
- A proxy's username or password typed after its address now takes effect; before, requests went out without them and were refused.
