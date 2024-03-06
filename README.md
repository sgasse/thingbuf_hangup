## Examples that hang up

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin thingbuf_sendref
   Compiling thingbuf_hangup v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/thingbuf_sendref`
[2024-03-06T07:27:28Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312333, tv_nsec: 954222386 }
[2024-03-06T07:27:29Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312334, tv_nsec: 954313029 }
[2024-03-06T07:27:30Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312335, tv_nsec: 954272381 }
[2024-03-06T07:27:31Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312336, tv_nsec: 954678568 }
[2024-03-06T07:27:32Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312337, tv_nsec: 954441415 }
[2024-03-06T07:27:33Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312338, tv_nsec: 954652616 }
[2024-03-06T07:27:34Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312339, tv_nsec: 954603474 }
[2024-03-06T07:27:35Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312340, tv_nsec: 954525381 }
[2024-03-06T07:27:36Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312341, tv_nsec: 954680718 }
[2024-03-06T07:27:37Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312342, tv_nsec: 955003492 }
[2024-03-06T07:27:38Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312343, tv_nsec: 954488259 }
[2024-03-06T07:27:38Z DEBUG thingbuf_sendref] Slowdown starting...
[2024-03-06T07:27:39Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312344, tv_nsec: 954362061 }
[2024-03-06T07:27:40Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312345, tv_nsec: 954858208 }
[2024-03-06T07:27:41Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312346, tv_nsec: 954959563 }
[2024-03-06T07:27:42Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312347, tv_nsec: 954179710 }
[2024-03-06T07:27:43Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312348, tv_nsec: 954195858 }
[2024-03-06T07:27:44Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312349, tv_nsec: 954872296 }
[2024-03-06T07:27:45Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312350, tv_nsec: 954844400 }
[2024-03-06T07:27:46Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312351, tv_nsec: 954451807 }
[2024-03-06T07:27:47Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312352, tv_nsec: 954629606 }
[2024-03-06T07:27:48Z INFO  thingbuf_sendref] Still alive: Instant { tv_sec: 312353, tv_nsec: 954238845 }
[2024-03-06T07:27:48Z DEBUG thingbuf_sendref] Starting async feeding...

(nothing printed after that point)
```

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin thingbuf_sendref_pure
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/thingbuf_sendref_pure`
[2024-03-06T07:28:36Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312401, tv_nsec: 478016458 }
[2024-03-06T07:28:37Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312402, tv_nsec: 478312435 }
[2024-03-06T07:28:38Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312403, tv_nsec: 478937522 }
[2024-03-06T07:28:39Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312404, tv_nsec: 478261920 }
[2024-03-06T07:28:40Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312405, tv_nsec: 478143752 }
[2024-03-06T07:28:41Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312406, tv_nsec: 478579681 }
[2024-03-06T07:28:42Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312407, tv_nsec: 478739247 }
[2024-03-06T07:28:43Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312408, tv_nsec: 478491277 }
[2024-03-06T07:28:44Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312409, tv_nsec: 478408646 }
[2024-03-06T07:28:45Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312410, tv_nsec: 478483671 }
[2024-03-06T07:28:46Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312411, tv_nsec: 478829183 }
[2024-03-06T07:28:46Z DEBUG thingbuf_sendref_pure] Slowdown starting...
[2024-03-06T07:28:47Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312412, tv_nsec: 478207678 }
[2024-03-06T07:28:48Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312413, tv_nsec: 478332472 }
[2024-03-06T07:28:49Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312414, tv_nsec: 478359378 }
[2024-03-06T07:28:50Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312415, tv_nsec: 478094714 }
[2024-03-06T07:28:51Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312416, tv_nsec: 478841302 }
[2024-03-06T07:28:52Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312417, tv_nsec: 478292986 }
[2024-03-06T07:28:53Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312418, tv_nsec: 478085688 }
[2024-03-06T07:28:54Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312419, tv_nsec: 478409977 }
[2024-03-06T07:28:55Z INFO  thingbuf_sendref_pure] Still alive: Instant { tv_sec: 312420, tv_nsec: 478210744 }
[2024-03-06T07:28:56Z DEBUG thingbuf_sendref_pure] Starting async feeding...

(nothing printed after that point)
```

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin thingbuf_send_recvref
   Compiling thingbuf_hangup v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/thingbuf_send_recvref`
[2024-03-06T08:31:20Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316165, tv_nsec: 374087708 }
[2024-03-06T08:31:21Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316166, tv_nsec: 374730026 }
[2024-03-06T08:31:22Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316167, tv_nsec: 374896516 }
[2024-03-06T08:31:23Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316168, tv_nsec: 374726066 }
[2024-03-06T08:31:24Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316169, tv_nsec: 374027193 }
[2024-03-06T08:31:25Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316170, tv_nsec: 374235150 }
[2024-03-06T08:31:26Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316171, tv_nsec: 374856434 }
[2024-03-06T08:31:27Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316172, tv_nsec: 374653910 }
[2024-03-06T08:31:28Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316173, tv_nsec: 374669724 }
[2024-03-06T08:31:29Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316174, tv_nsec: 374108117 }
[2024-03-06T08:31:30Z DEBUG thingbuf_send_recvref] Slowdown starting...
[2024-03-06T08:31:30Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316175, tv_nsec: 374461717 }
[2024-03-06T08:31:31Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316176, tv_nsec: 374055222 }
[2024-03-06T08:31:32Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316177, tv_nsec: 374820471 }
[2024-03-06T08:31:33Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316178, tv_nsec: 374033907 }
[2024-03-06T08:31:34Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316179, tv_nsec: 374615812 }
[2024-03-06T08:31:35Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316180, tv_nsec: 374451224 }
[2024-03-06T08:31:36Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316181, tv_nsec: 374290534 }
[2024-03-06T08:31:37Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316182, tv_nsec: 374742021 }
[2024-03-06T08:31:38Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316183, tv_nsec: 374805845 }
[2024-03-06T08:31:39Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316184, tv_nsec: 374294119 }
[2024-03-06T08:31:40Z INFO  thingbuf_send_recvref] Still alive: Instant { tv_sec: 316185, tv_nsec: 374514728 }
[2024-03-06T08:31:40Z DEBUG thingbuf_send_recvref] Starting async feeding...

(nothing printed after that point)
```

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin thingbuf_send_no_try_recvref
   Compiling thingbuf_hangup v0.1.0 (...)
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/thingbuf_send_no_try_recvref`
[2024-03-06T08:37:35Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316540, tv_nsec: 970038923 }
[2024-03-06T08:37:36Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316541, tv_nsec: 969999285 }
[2024-03-06T08:37:37Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316542, tv_nsec: 970752111 }
[2024-03-06T08:37:38Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316543, tv_nsec: 970710066 }
[2024-03-06T08:37:39Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316544, tv_nsec: 970613752 }
[2024-03-06T08:37:40Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316545, tv_nsec: 969890234 }
[2024-03-06T08:37:41Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316546, tv_nsec: 970813811 }
[2024-03-06T08:37:42Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316547, tv_nsec: 970618987 }
[2024-03-06T08:37:43Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316548, tv_nsec: 970347568 }
[2024-03-06T08:37:44Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316549, tv_nsec: 970413945 }
[2024-03-06T08:37:45Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316550, tv_nsec: 970815619 }
[2024-03-06T08:37:45Z DEBUG thingbuf_send_no_try_recvref] Slowdown starting...
[2024-03-06T08:37:46Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316551, tv_nsec: 970252004 }
[2024-03-06T08:37:47Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316552, tv_nsec: 970847633 }
[2024-03-06T08:37:48Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316553, tv_nsec: 970367718 }
[2024-03-06T08:37:49Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316554, tv_nsec: 969911982 }
[2024-03-06T08:37:50Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316555, tv_nsec: 970736282 }
[2024-03-06T08:37:51Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316556, tv_nsec: 970495000 }
[2024-03-06T08:37:52Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316557, tv_nsec: 970057594 }
[2024-03-06T08:37:53Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316558, tv_nsec: 970717780 }
[2024-03-06T08:37:54Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316559, tv_nsec: 970015553 }
[2024-03-06T08:37:55Z INFO  thingbuf_send_no_try_recvref] Still alive: Instant { tv_sec: 316560, tv_nsec: 970613333 }
[2024-03-06T08:37:55Z DEBUG thingbuf_send_no_try_recvref] Starting async feeding...

(nothing printed after that point)
```

## Examples that work as expected

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin thingbuf_send                                                                                                                                                         03/06/2024 08:32:47 AM
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/thingbuf_send`
[2024-03-06T07:32:59Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312664, tv_nsec: 536877693 }
[2024-03-06T07:33:00Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312665, tv_nsec: 537135375 }
[2024-03-06T07:33:01Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312666, tv_nsec: 537618556 }
[2024-03-06T07:33:02Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312667, tv_nsec: 537378695 }
[2024-03-06T07:33:03Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312668, tv_nsec: 536940184 }
[2024-03-06T07:33:04Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312669, tv_nsec: 537380105 }
[2024-03-06T07:33:05Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312670, tv_nsec: 536992759 }
[2024-03-06T07:33:06Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312671, tv_nsec: 537218194 }
[2024-03-06T07:33:07Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312672, tv_nsec: 537599941 }
[2024-03-06T07:33:08Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312673, tv_nsec: 536754724 }
[2024-03-06T07:33:09Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312674, tv_nsec: 537273035 }
[2024-03-06T07:33:09Z DEBUG thingbuf_send] Slowdown starting...
[2024-03-06T07:33:10Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312675, tv_nsec: 537397733 }
[2024-03-06T07:33:11Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312676, tv_nsec: 537404395 }
[2024-03-06T07:33:12Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312677, tv_nsec: 536693173 }
[2024-03-06T07:33:13Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312678, tv_nsec: 537340827 }
[2024-03-06T07:33:14Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312679, tv_nsec: 536737003 }
[2024-03-06T07:33:15Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312680, tv_nsec: 537013987 }
[2024-03-06T07:33:16Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312681, tv_nsec: 537498879 }
[2024-03-06T07:33:17Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312682, tv_nsec: 537514333 }
[2024-03-06T07:33:18Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312683, tv_nsec: 536817566 }
[2024-03-06T07:33:19Z DEBUG thingbuf_send] Starting async feeding...
[2024-03-06T07:33:19Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312684, tv_nsec: 536901829 }
[2024-03-06T07:33:19Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:19Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:19Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:19Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:19Z WARN  thingbuf_send] Channel full. Waiting...
...
[2024-03-06T07:33:20Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:20Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312685, tv_nsec: 536841781 }
[2024-03-06T07:33:20Z WARN  thingbuf_send] Channel full. Waiting...
...
[2024-03-06T07:33:21Z WARN  thingbuf_send] Channel full. Waiting...
[2024-03-06T07:33:21Z INFO  thingbuf_send] Still alive: Instant { tv_sec: 312686, tv_nsec: 537362322 }
[2024-03-06T07:33:21Z WARN  thingbuf_send] Channel full. Waiting...
...
[2024-03-06T07:33:21Z WARN  thingbuf_send] Channel full. Waiting...
...
```

```
thingbuf_hangup〉RUST_LOG=debug cargo run --bin tokio_mpsc                                                                                                                                                            03/06/2024 08:33:22 AM
   Compiling thingbuf_hangup v0.1.0 (/home/simon-gasse/workspace/examples/thingbuf_hangup)
    Finished dev [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/tokio_mpsc`
[2024-03-06T07:35:02Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312787, tv_nsec: 429111852 }
[2024-03-06T07:35:03Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312788, tv_nsec: 429766280 }
[2024-03-06T07:35:04Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312789, tv_nsec: 429528550 }
[2024-03-06T07:35:05Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312790, tv_nsec: 429512975 }
[2024-03-06T07:35:06Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312791, tv_nsec: 429923649 }
[2024-03-06T07:35:07Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312792, tv_nsec: 429498437 }
[2024-03-06T07:35:08Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312793, tv_nsec: 429314472 }
[2024-03-06T07:35:09Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312794, tv_nsec: 429967324 }
[2024-03-06T07:35:10Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312795, tv_nsec: 429530545 }
[2024-03-06T07:35:11Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312796, tv_nsec: 429763858 }
[2024-03-06T07:35:12Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312797, tv_nsec: 429041977 }
[2024-03-06T07:35:12Z DEBUG tokio_mpsc] Slowdown starting...
[2024-03-06T07:35:13Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312798, tv_nsec: 429294836 }
[2024-03-06T07:35:14Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312799, tv_nsec: 429742823 }
[2024-03-06T07:35:15Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312800, tv_nsec: 429574871 }
[2024-03-06T07:35:16Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312801, tv_nsec: 429717193 }
[2024-03-06T07:35:17Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312802, tv_nsec: 429106822 }
[2024-03-06T07:35:18Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312803, tv_nsec: 429302232 }
[2024-03-06T07:35:19Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312804, tv_nsec: 429913042 }
[2024-03-06T07:35:20Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312805, tv_nsec: 429307881 }
[2024-03-06T07:35:21Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312806, tv_nsec: 429254677 }
[2024-03-06T07:35:22Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312807, tv_nsec: 429099865 }
[2024-03-06T07:35:22Z DEBUG tokio_mpsc] Starting async feeding...
[2024-03-06T07:35:22Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:22Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:22Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:22Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:22Z WARN  tokio_mpsc] Channel full. Waiting...
...
[2024-03-06T07:35:23Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:23Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312808, tv_nsec: 429845893 }
[2024-03-06T07:35:23Z WARN  tokio_mpsc] Channel full. Waiting...
...
[2024-03-06T07:35:24Z WARN  tokio_mpsc] Channel full. Waiting...
[2024-03-06T07:35:24Z INFO  tokio_mpsc] Still alive: Instant { tv_sec: 312809, tv_nsec: 429914811 }
[2024-03-06T07:35:24Z WARN  tokio_mpsc] Channel full. Waiting...
...
```

## Logs

Healthy

```
[2023-03-06T08:15:06Z WARN  thingbuf::mpsc::async_impl] Try send_ref
[2023-03-06T08:15:06Z DEBUG thingbuf] thread ThreadId(3) line 196: push_ref entry
[2023-03-06T08:15:06Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2023-03-06T08:15:06Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 22, state: 5142
[2023-03-06T08:15:06Z WARN  thingbuf] Next tail: 5143
[2023-03-06T08:15:06Z DEBUG thingbuf] thread ThreadId(3) line 240: Claimed tail slot [22]
```

Unhealthy / hung-up

```
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 202: push_ref loop starts
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 227: idx: 77, state: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 294: if
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(3) line 286: head: 7246
[2024-03-06T08:16:20Z DEBUG thingbuf] thread ThreadId(2) line 301: tail at the end of the loop: 7757
```
