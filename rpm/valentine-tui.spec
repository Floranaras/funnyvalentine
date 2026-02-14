Name:           valentine-tui
Version:        0.1.0
Release:        1%{?dist}
Summary:        A playful terminal-based Valentine's Day proposal app

License:        MIT
URL:            https://github.com/Floranaras/funnyvalentine
Source0:        %{url}/archive/v%{version}/funnyvalentine-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  alsa-lib-devel

%description
A playful terminal-based Valentine's Day proposal app built with Rust and ratatui.

%prep
%autosetup -n funnyvalentine-%{version}

%build
cargo build --release --locked

%install
install -Dm755 target/release/valentine-tui %{buildroot}%{_bindir}/valentine-tui

%files
%license LICENSE
%{_bindir}/valentine-tui

%changelog
* Fri Feb 14 2026 Amane Kai - 0.1.0-1
- Initial release
