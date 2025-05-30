<p style="text-align: right"> 
    <a href="./README.md">[INDEX]</a> &#9;&#9;
    <a href="./S01_00_Rust시작.md">[S01. Rust 시작]</a>
</p>

# S01. Rust 시작

* [1-1.][S01_01_Rust설치] 각 운영체제 (Linux, macOS, Windows) 별 러스트 설치법 
* [1-2.][S01_02_H_World] `Hello, World!` 프로그램 작성하기
* [1-3.][S01_03_H_Cargo] 러스트 패키지 매니저 및 빌드 도구인 `cargo` 사용법

[S01_01_Rust설치]: ./S01_01_Rust설치.md
[S01_02_H_World]: ./S01_02_Hello_World.md
[S01_03_H_Cargo]: ./S01_03_Hello_Cargo.md

---
### 1-2. Hello, World!

설치도 마쳤으니, 러스트 프로그램을 만들 시간입니다.
새 언어를 배울 때면 늘 그렇듯,
만들어 볼 프로그램은 화면에 `Hello, World!` 문자를 출력하는 간단한 프로그램입니다.

> Note:
> 이 책은 커맨드 라인 위주로 설명하고 있습니다.
> 하지만 러스트에는 코드 작성 및 개발 도구 사용환경에 따로 정해진 규정이 없으므로
> 커맨드 라인 대신 IDE (통합 개발 환경) 를 사용하실 분은 애용하는 IDE를 사용하셔도 좋습니다.
> (요즘은 IDE 대부분이 러스트를 어느 정도 지원하니 세부 사항은 각 IDE 문서를 참고 바랍니다)
> 러스트 팀은 `rust-analyzer`를 통하여 IDE 지원 수준을 높이는 데 집중하고 있습니다.
> 더 자세한 사항은 [부록 D][devtools]<!-- ignore -->를 참고하세요.

### 프로젝트 디렉터리 생성하기

작성할 러스트 코드를 저장해 둘 디렉터리가 필요하겠죠.
러스트 코드 자체는 어디에 저장하건 실행하는 데 문제는 없습니다만,
이 책을 보며 연습하시는 분들은 편의를 위해 홈 디렉터리 내 *projects*
디렉터리를 생성해 각종 프로젝트를 보관하는 것을 권장해 드립니다.

터미널을 열고 다음 명령어를 입력해 *projects* 디렉터리를 생성한 후,
*projects* 내에 ‘Hello, World!’ 프로젝트용 디렉터리를 만들어 봅시다.

Linux, macOS, Windows PowerShell에서는 다음 명령어를 입력해 주세요:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD 사용자는 다음 명령어를 입력해 주세요:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### 러스트 프로그램 작성하고 실행하기

다음으로 *main.rs* 소스 파일을 만들어 봅시다. 러스트 파일은 항상
*.rs* 확장자로 끝납니다. 파일명을 지을 때는 두 단어 이상으로 이루어질 경우에는
*helloworld.rs* 와 같이 붙여서 쓰지 않고 *hello_world.rs*처럼 단어 사이에
밑줄 (`_`) 을 넣는 것이 관례입니다.

*main.rs* 파일에 예제 1-1 코드를 입력합시다.

<span class="filename">파일명: main.rs</span>

```rust
fn main() {
  println!("Hello, World!");
}
```

<span class="caption">예제 1-1: `Hello, World!`를 출력하는 프로그램</span>

파일을 저장하고 터미널 창으로 돌아가 *~/projects/hello_world*
디렉터리로 갑니다. Linux, macOS 사용자는 다음 명령어를 입력하여
컴파일하고 실행할 수 있습니다:

```console
$ rustc main.rs
$ ./main
Hello, World!
```

Windows에서는 `./main`을 `.\main.exe`로 바꿔주시면 됩니다:

```powershell
> rustc main.rs
> .\main.exe
Hello, World!
```

사용하시는 운영체제와 상관없이
터미널에 `Hello, World!`가 출력되면 정상입니다.
출력되지 않으면 [‘트러블 슈팅’][troubleshooting]<!-- ignore -->
내용을 참고해 도움을 얻을 방법을 찾아보세요.

문제없이 `Hello, World!`가 출력됐다면, 축하드립니다! 여러분은 공식적으로
러스트 프로그램을 작성했으니 이제 어엿한 러스트 프로그래머입니다!

### 러스트 프로그램 뜯어보기

방금 만든 ‘Hello, World!’ 프로그램을 자세히 살펴봅시다.
우선 첫 부분은 다음과 같습니다:

```rust
fn main() {

}
```

이 라인은 러스트에서 `main`이라는 이름의 함수를 정의합니다. `main` 함수는
특별한 함수로, 러스트 실행 프로그램에서 항상 가장 먼저 실행되는 함수입니다.
여기서는 매개변수를 받지 않고 아무것도 반환하지 않는 `main`이라는 함수를 선언합니다.
함수에 매개변수가 있을 때는 `()` 안쪽에 이를 작성해야 합니다.

함수 본문은 `{}`로 감싸집니다. 러스트에서는 모든 함수에 대해 본문을 감싸는
중괄호(`{}`)가 필수입니다. 중괄호는 함수 정의와 같은 줄에 작성하고
그 사이에 공백을 한 칸 넣으면 보기 좋으니 참고하세요.

> Note: 여러분이 러스트 프로젝트의 코드를 표준 스타일로 통일시키고 싶다면,
> 코드를 특정 스타일로 포맷팅해주는 `rustfmt`라는 이름의 자동 포맷팅 도구를
> 사용할 수 있습니다 (더 자세한 사항은
> [부록 D][devtools]<!-- ignore -->에 있습니다.)
> 러스트 팀은 이 도구를 `rustc`처럼 기본 러스트 배포에
> 포함시켰으므로, 이미 여러분의 컴퓨터에 설치되어 있습니다!

`main` 함수 내 코드를 살펴봅시다.

```rust
    println!("Hello, World!");
```

화면에 텍스트를 출력하는 코드로, 이 한 라인이 이 자그마한 프로그램의 전부입니다.
하지만 이 단순한 코드에도 눈여겨볼 것이 네 가지 들어있습니다.

첫 번째로, 러스트에서는 탭 대신 스페이스 4칸을 사용합니다.

두 번째로, `println!`는 러스트의 매크로 호출 코드입니다. 이 코드가
함수 호출 코드였다면 `!` 없이 `println`이라고 되어 있었을 것입니다.
매크로는 19장에서 자세히 다루며, 지금은 `!`가 붙으면 함수가 아니라
매크로 호출 코드이고, 매크로는 함수와 항상 같은 규칙을 따르지는 않는다는
것만 알아두시면 됩니다.

세 번째는 `println!`의 인수로 넘겨준 `"Hello, World!"` 문자열이
그대로 화면에 나타난 점입니다.

마지막으로, 이 라인은 세미콜론(`;`)으로 끝납니다. 이 표현식이
끝났으며 다음 표현식이 시작될 준비가 됐다는 표시지요. 러스트
코드의 거의 모든 라인이 세미콜론으로 끝납니다.

### 컴파일과 실행은 별개의 과정입니다

앞서 새 프로그램을 만들고 실행한 과정을
세세한 단계로 나누어 검토해 봅시다.

러스트 프로그램을 실행하기 전에, 아래와 같이 `rustc`
명령어에 소스 파일명을 넘겨주어 컴파일해야 하는 과정이
있었습니다:

```console
$ rustc main.rs
```

C나 C++ 을 다뤄보셨다면 `gcc`나 `clang` 사용 방법과 비슷하다는 걸 눈치채셨을지도 모르겠네요.
러스트는 소스 파일 컴파일에 성공하면 실행 가능한 바이너리를 만들어 냅니다.

Linux, macOS, Windows PowerShell 상에서는
`ls` 명령어로 실행 파일을 확인할 수 있습니다.

```console
$ ls
main  main.rs
```

Linux와 macOS에서는 두 개의 파일이 보일 것이고, PowerShell의 경우에는
CMD와 같이 세 개의 파일이 보일 것입니다. Windows CMD 는 다음 명령어를
입력해야 합니다:

```cmd
> dir /B %= `/B`는  파일명만 출력하는 옵션입니다 =%
main.exe
main.pdb
main.rs
```

*.rs* 확장자를 갖는 소스 파일과 실행 파일
(타 플랫폼에서는 *main*, Windows에서는 *main.exe*입니다)을 확인할 수 있습니다.
Windows에서는 디버깅 정보가 포함된 *pdb* 확장자 파일도 볼 수 있네요.
여기서 *main* 이나 *main.exe* 를 실행하는 방법은 다음과 같습니다:

```text
$ ./main # Windows에서는 .\main.exe
```

*main.rs*가 여러분의 ‘Hello, World!’ 프로그램이라면
터미널에 `Hello, World!`가 출력될 겁니다.

Ruby, Python, JavaScript 등 명령어 한 줄로 프로그램을 컴파일하고
실행할 수 있는 동적 프로그래밍 언어에 익숙한 분들은 컴파일과 실행이
별개의 과정으로 진행되는 게 낯설 겁니다. 하지만 이 언어들은
*.rb*, *.py*, *.js* 파일을 다른 곳에서 실행하려면 해당 언어의 구현체를 설치해야만 합니다.
반면 러스트는 *AOT(ahead-of-time-compiled)* 언어로,
컴파일과 실행이 별개인 대신 여러분의 프로그램을 컴파일하여 만든 실행 파일을
러스트가 설치되지 않은 곳에서도 실행할 수 있습니다.
저마다 장단점이 있는 법이죠.

간단한 프로그램에는 `rustc`를 사용하는 것도 좋습니다.
다만 프로젝트가 커질수록 관리할 옵션이 많아지고, 코드 배포도 점점 번거로워지겠죠.
다음 내용에서 소개할 카고 (Cargo) 가 바로 이러한 문제를 해결하는,
여러분이 앞으로 `rustc` 대신 사용할 도구입니다.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.md

