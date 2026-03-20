# Biologie memory

Een memory spel voor mijn biologie project.

## Doelen
- [ ] goed cijfer voor po :)
- [ ] material you (-ish) ontwerp
- [ ] uiteindelijk ook voor andere vakken te kunnen gebruiken
- [ ] windows

## Windows
Voor windows heb je msvc nodig, rustup (installeert ook msvc) en qt VOOR MSVC.
Dan doe je:
```powershell
$env:PATH += ";C:\Qt\6.10.2\msvc2022_64\bin"
$env:RUSTFLAGS="-L C:\Qt\6.10.2\msvc2022\lib" # replace 6.10.2 with your qt version
cargo build --release
```

## License
Copyright (C) 2026 AshyPinguin

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Mogelijk andere namen

### CuteM

Cute want qt, m van memory
