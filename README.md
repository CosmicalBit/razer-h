<div align="center">
  <h1>🖱️ razer-h</h1>
  <p><strong>A tiny Linux utility for applying Razer mouse DPI and polling-rate settings directly over USB.</strong></p>
  <p>
    <img alt="Rust 2024" src="https://img.shields.io/badge/Rust-2024-DEA584?logo=rust&amp;logoColor=white">
    <img alt="Linux" src="https://img.shields.io/badge/platform-Linux-FCC624?logo=linux&amp;logoColor=black">
    <img alt="libusb" src="https://img.shields.io/badge/USB-libusb-00599C">
    <img alt="Polling rate up to 8 kHz" src="https://img.shields.io/badge/polling-up%20to%208000%20Hz-2EA44F">
  </p>
  <p>No GUI, daemon, or background service: read a small config file, send the HID reports, and exit.</p>
</div>

<hr>

<h2>What it does</h2>

<table>
  <tbody>
    <tr>
      <td>🎯 <strong>DPI</strong></td>
      <td>Applies the same non-zero <code>u16</code> DPI value to the X and Y axes.</td>
    </tr>
    <tr>
      <td>⚡ <strong>Polling</strong></td>
      <td>Supports 125, 250, 500, 1000, 2000, 4000, and 8000 Hz.</td>
    </tr>
    <tr>
      <td>💾 <strong>Onboard profiles</strong></td>
      <td>Sends polling reports for both onboard selectors used by the current protocol implementation.</td>
    </tr>
    <tr>
      <td>🪶 <strong>Small footprint</strong></td>
      <td>Uses <code>rusb</code> and exits after applying the settings.</td>
    </tr>
  </tbody>
</table>

<h2>Current device scope</h2>

<p>The detector currently accepts Razer vendor ID <code>1532</code> with these two product IDs:</p>

<table>
  <thead>
    <tr>
      <th align="left">Connection</th>
      <th align="left">USB ID</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Wired</td>
      <td><code>1532:00c0</code></td>
    </tr>
    <tr>
      <td>Wireless</td>
      <td><code>1532:00c1</code></td>
    </tr>
  </tbody>
</table>

<blockquote>
  <p><strong>Hardware warning:</strong> protocol commands are device-specific. Do not assume another Razer mouse is compatible merely because it shares the vendor ID.</p>
</blockquote>

<h2>Build</h2>

<p>Install a Rust toolchain and your distribution's libusb development package, then build:</p>

<pre><code>cargo build --release</code></pre>

<p>The binary is written to <code>target/release/razer-h</code>.</p>

<h2>Configure and run</h2>

<p>On the first run, <code>razer-h</code> asks for a polling rate and DPI, then creates:</p>

<pre><code>~/.config/razer-h</code></pre>

<p>The file format is deliberately simple:</p>

<pre><code>dpi=1600
poll_rate=8000</code></pre>

<p>Run without arguments to load and apply the saved values:</p>

<pre><code>./target/release/razer-h</code></pre>

<p>If a saved value is missing or invalid, the current fallback is <code>1600</code> DPI and <code>8000</code> Hz. Command-line arguments are not implemented yet.</p>

<h2>USB permissions</h2>

<p>The process must be allowed to open the device and claim HID interface <code>2</code>. If it reports a permission or claim error, configure an appropriate udev rule for the exact device IDs above or run it temporarily with sufficient privileges while testing.</p>

<h2>Development</h2>

<pre><code>cargo fmt --all --check
cargo test
cargo clippy --all-targets</code></pre>

<h2>Project status</h2>

<p>This is an early, intentionally narrow utility. Detection is limited to two product IDs, settings are file-based, and there is no CLI parser or broad device database yet.</p>

<div align="center">
  <sub>Set it once. Keep the stack small.</sub>
</div>
