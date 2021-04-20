<template>
    <div id="side">
        <main>
            <div id="about">
                <div id="title">
                    <span id="img"><img src="../../../static/imgs/kriry.png"></span>
                    <span id="mean">&nbsp;：探索-互动</span>
                </div>
                <div id="line"></div>
                <div id="content">
                    <p>欢迎来到kriry,&nbsp;让我们一起在探索与互动中收获更多.&nbsp;<a>关于kriry</a></p>
                </div>
            </div>

            <div id="site">
                <div id="title">
                    <span id="arrow"></span><span id="word"> 友情链接</span>
                </div>
                <div id="line"></div>
                <div id="content">
                    <ul>
                        <li><a href="https://dev.kriry.com/" target="_black"><strong>kriry</strong>技术</a></li>
                        <li>微博 : <a href="https://weibo.com/kriry?is_all=1" target="_black">kriry</a></li>
                        <li>微博 : <a href="https://weibo.com/FutureFellow?is_all=1" target="_black">FutureFellow</a></li>
                        <li>微信公众号 : <sapn style="font-size:1rem;">kriry</sapn></li>
                        <li>微信公众号 : <sapn style="font-size:1rem;">FutureFellow</sapn></li>
                        <li>知乎专栏 : <a href="https://zhuanlan.zhihu.com/futurefellow" target="_black">FutureFellow</a></li>
                        <li>QQ群 : 951882729</li>
                    </ul>
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'side'
}
</script>

<style scoped>
main #site, main #about {
    margin-bottom: 0.6rem;
    box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
}
main a {
    color: #0541af;
}
main  #line {
    border-bottom: 1px solid #ddd;
}
main  #content {
    font-size: 0.9rem;
    padding: 0.3rem 1rem;
}
main #site #title {
    padding: 0.3rem 0.5rem;
}
main #site #arrow {
        display:inline-block;
        border-top: 10px solid transparent;
        border-left: 10px solid #8013e6;
        border-bottom: 10px solid transparent;
}
main #site #word {
        padding-bottom: 1.1rem;
}
main #site #content ul a {
        font-size: 1rem;
}
main #about #title {
    display: flex;
    padding: 0.3rem 0.5rem 0;
}
main #about #title img {
    height: 2rem;
    width: 4.4rem;
}
main #about #title #mean {
    font-size: 1.1rem;
    padding-top: 0.3rem;
}
@media only screen and (min-width: 1000px) {
    main {
        width: 15rem;
    }
}
</style>