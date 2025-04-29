<p style="text-align: right"> 
    <a href="./README.md">[INDEX]</a>
</p>

# S01. Rust 시작

* [01][S01_01_Rust설치    ] 각 운영체제 (Linux, macOS, Windows) 별 러스트 설치법 
* [02][S01_02_Hello_World] `Hello, world!` 프로그램 작성하기
* [러스트 패키지 매니저 및 빌드 도구인 `cargo` 사용법      ][S01_01_Rust설치]

[S01_01_Rust설치]: ./S01_01_Rust설치.md
[S01_02_H_World]: ./S01_02_Hello_World.md
[S01_03_H_Cargo]: ./S01_03_Hello_Cargo.md

---
### 1. Rust 설치

> ### 커맨드 라인 표기
>
> 이번 장을 비롯해 터미널에 명령어를 입력할 일이 많습니다.
> 입력할 명령어와 출력을 구분하실 수 있도록, 명령어에는
> 각 행 앞에 `$`가 붙습니다. `$`가 붙지 않은 행은
> 보통 앞선 명령어의 결과를 나타낸다고 보시면 됩니다.
> 예외적으로, `$` 대신 `>`가 붙은 예제는
> PowerShell 한정 예제입니다.

### `rustup` 설치 - Linux 및 macOS

Linux 나 macOS 사용자는 터미널을 열고 다음 명령어를 입력해 주세요:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

최신 stable 버전 러스트를 설치하는 데 사용할 `rustup` 도구를 설치하는
명령어입니다. (설치할 때 여러분 비밀번호를 묻는 메시지가 나타날 수 있습니다.)
설치가 완료되면 다음 문장이 나타납니다:

```text
Rust is installed now. Great!
```

링커는 기본으로 설치되나, 러스트 컴파일 시에 링커를
실행할 수 없다는 에러가 나타나면 따로 설치하셔야 합니다.
이 에러는 C 컴파일러를 설치할 때 같이 설치되는 링커로 해결되므로
플랫폼에 맞는 C 컴파일러를 찾아서 설치하시기 바랍니다. 몇 가지 흔히 사용되는
러스트 패키지들이 C 코드를 이용하고 있기 때문에 C 컴파일러가 필요할 수도 있습니다.

macOS에서는 아래와 같이 실행하여 C 컴파일러를 설치할 수 있습니다:

```console
$ xcode-select --install
```

Linux 사용자의 경우 배포판의 문서에 의하면 일반적으로 GCC나 Clang이
설치되어 있습니다. 예를 들어 우분투 사용자라면 `build-essential` 패키지를
설치할 수 있습니다.

### `rustup` 설치 - Windows

Windows 사용자는 [https://www.rust-lang.org/tools/install][rust-install]  에서 안내를 따라주시기 바랍니다. 설치 과정에서 Visual Studio 2013 버전 이상의 MSVC 빌드 도구가 필요하다는 메시지가 나타날 것입니다.

빌드 도구를 설치하려면 [Visual Studio 2022][visualstudio] 를 설치할 필요가 있습니다. 구체적으로는 아래와 같은 패키지가 필요합니다:

* ‘C++ 데스크톱 개발’
* Windows 10 혹은 11 SDK
* 영어 언어팩과 여러분이 선택하고 싶은 다른
  언어팩

이후부터는 *cmd.exe* 와 PowerShell에서 혼용되는 명령어만 사용할 예정이며, 서로 다른 부분이 있을 경우엔 따로 명시하겠습니다.

### 트러블 슈팅

러스트가 제대로 설치되었는지 확인하는 방법은 다음과 같습니다:

```console
$ rustc --version
```

최신 릴리즈된 stable 버전 정보가 다음 포맷대로 나타나며, 
나타난 정보는 순서대로 버전 숫자, 커밋 해시 (hash), 커밋 날짜입니다:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

위의 정보가 보이면 러스트가 성공적으로 설치된 것입니다!
정보가 보이지 않는다면 여러분의 `%PATH%` 시스템 변수에 러스트가 포함되어 있는지 확인해 주세요.

Windows CMD에서는 다음과 같이 확인합니다:

```console
> echo %PATH%
```

PowerShell에서는 다음과 같이 확인합니다:

```powershell
> echo $env:Path
```

Linux와 macOS에서는 다음과 같이 확인합니다:

```console
$ echo $PATH
```

잘못된 것을 찾을 수 없는데 계속 작동하지 않으면 [한국 러스트 사용자 그룹 디스코드][korean_discord]에 질문해 주세요.
영어가 능숙한 분들은 [커뮤니티 페이지][community]에서 다른 러스타시안 (Rustacean, 러스트 사용자들 스스로를
부르는 웃긴 별명입니다) 들을 만나볼 수 있을 겁니다.

### 업데이트 및 삭제

`rustup`으로 러스트를 설치했다면 최신 버전 업데이트도 간편합니다.
셸에 다음 명령어를 입력해 주세요:

```console
$ rustup update
```

`rustup`과 러스트를 삭제하는 방법은 다음과
같습니다:

```console
$ rustup self uninstall
```

### 로컬 문서

러스트 설치 시 로컬 문서 (local documentation) 도 같이 설치됩니다. 오프라인
상태로도 이용할 수 있으며, `rustup doc` 명령어로 여러분의 브라우저에서 열어볼 수 있습니다.

표준 라이브러리에서 제공하는 타입이나 함수 중 이게 무슨 기능을 하는지나
사용하는 법을 모르겠다면 API (Application Programming Language) 문서에서
모르는 내용을 찾아볼 수도 있겠죠?


[rust-install  ]: https://www.rust-lang.org/tools/install
[visualstudio  ]: https://visualstudio.microsoft.com/ko/downloads/
[korean_discord]: https://discord.com/invite/uqXGjEz
[community     ]: https://www.rust-lang.org/community
