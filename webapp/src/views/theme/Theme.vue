<template>
    <div id="theme">
        <main id="main">
            <div id="container">
                <div id="center">
                    <div id="theme">
                        <div id="title">
                            <div id="theme-title"> {{ theme.title }} </div> 
                            <span id="info"><a :href="'/a/community/' + theme_community_name">{{ theme_community_name_cn }}</a></span> • 
                            <span id="info"><a :href="'/a/community/' + theme_community_name + '/' + theme.community_tag_name">{{ theme.community_tag_name }}</a></span> • 
                            <span id="info"><a :href="'/a/user/' + theme_user.id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme_rtime }}</span> •&nbsp;
                            <span v-if="signin_user.username == theme_user.username" id="info">
                                <a :href="'/a/'+ theme_community_name + '/theme/edit/' + theme.id">编辑 •&nbsp;</a>
                            </span> 
                            <span v-if="theme_save == false" id="save" @click="save">收藏</span>
                            <span v-if="theme_save == true" id="saved">已收藏</span>
                        </div>
                        <div id="content" v-html="theme.content" v-highlight> </div>
                    </div>
                    <hr>
                    <div id="comment">
                        <div id="count">评论 &nbsp; {{ theme.comment_count }} </div>
                        <div v-for="(comment, index) in theme_comments" :key="index">
                            <div id="detail">
                                <div id="avatar">
                                    <a :href="'/a/user/' + comment.user.id"><img :src= comment.user.avatar ></a>
                                </div>
                                <div id="comment_item"> 
                                    <div id="infos">
                                        <span id="info" >{{ index + 1 }}&nbsp;</span>
                                        <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.user.username }}</a></span> • 
                                        <span id="info">{{ comment.rtime }}</span>
                                    </div>
                                    <div id="content" v-html="comment.content" v-highlight> </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <hr>
                    <div id="reply" v-if="signin_user">
                        <div id="messagenote">
                            <p>注意发消息格式为<strong style="background-color: #ccc;">@人名 内容</strong>中间有一空格,不然发不出去或收不到</p>
                        </div>
                        <div id="editor">
                            <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                        </div>
                        <button style="margin: 0.2rem 0;
                                       width:66px; 
                                       line-height:25px;
                                       background-color:#ffffff;
                                       border :2.2px solid #a39c9c;" type="submit" id="submit" @click="comment">评论
                        </button>
                    </div>  
                    <div v-else style="padding: 0.8rem;">请先登录再发表评论.
                        <a href="/a/signin" style="background-color: #ccc; padding: 0.2rem;">登录</a>
                    </div>        
                </div>
                <div id="aside">
                    <side></side>
                </div>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import { mavonEditor } from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import URLprefix from '../../config'
import Side from '../../components/side/Side.vue'
export default {
    name: 'theme',
    components: {
        mavonEditor,
        "side": Side
    },
    data: function() {
        return {
            Content: '',
            theme: '',
            theme_user: '',
            theme_community_name: '',
            theme_community_name_cn: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: '',
            theme_save: false,
            set:{
                bold: true, // 粗体
                italic: true, // 斜体
                header: true, // 标题
                quote: true, // 引用
                ul: true, // 无序列表
                link: true, // 链接
                code: true, // code
                table: true, // 表格
                trash: true, // 清空
                fullscreen: true, // 全屏编辑
                htmlcode: true, // 展示html源码
                preview: true, // 预览
                help: true, // 帮助
                
                underline: false, // 下划线
                strikethrough: false, // 中划线
                mark: false, // 标记
                ol: false, // 有序列表
                 alignleft: false, // 左对齐
                aligncenter: false, // 居中
                alignright: false, // 右对齐
                superscript: false, // 上角标
                subscript: false, // 下角标
                undo: false, // 上一步
                redo: false, // 下一步
                imagelink: false, // 图片链接
                readmodel: false, // 沉浸式阅读
                save: false, // 保存（触发events中的save事件）
                navigation: false, // 导航目录
                subfield: false, // 单双栏模式
            }
        }
    },
    mounted: function() {
        let user_id = 0
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
            user_id = JSON.parse(localStorage.getItem('signin_user')).id
        }
        let theme_id = Number.parseInt(this.$route.params.id)
        let data = {
            theme_id: theme_id,
            user_id: Number.parseInt(user_id)
        }
        fetch(URLprefix + 'api/theme/'+ this.$route.params.id,{
                body: JSON.stringify(data), 
                headers: {
                  'content-type': 'application/json'
                },
                method: 'POST',
                mode: 'cors'
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            this.theme_community_name_cn = json.theme_community_name_cn
            this.theme_community_name = json.theme_community_name
            json.theme_comments.map((item) => {
                if (item.user.avatar == "") {
                    item.user.avatar = "https://www.gravatar.com/avatar/1"
                }
            })
            this.theme_comments = json.theme_comments
            this.theme_save = json.theme_save
        }).catch((e) => {
            console.log(e)
        })
  },
  methods: {
    comment () {
        let comment = this.Content
        let theme_id = this.$route.params.id
        let theme_user_id = this.theme_user.id
        let user_id = JSON.parse(localStorage.getItem('signin_user')).id
        let data = {
            theme_id: Number.parseInt(theme_id),
            theme_user_id: Number.parseInt(theme_user_id),
            user_id: Number.parseInt(user_id),
            comment: comment
        }
        fetch(URLprefix + "api/theme_comment/" + this.$route.params.id, {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
    },
    save() {
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            let save = document.getElementById("save")
            save.innerHTML = "已收藏"
            let user_id = JSON.parse(localStorage.getItem('signin_user')).id
            let theme_id = this.$route.params.id
            let data = { 
                theme_id: Number.parseInt(theme_id),
                user_id: Number.parseInt(user_id)
            }
            fetch(URLprefix + 'api/themeid/save', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
        }else{
            alert("请先登录.")
            // this.$router.push('/a/signin');
        }
    }
  }
}
</script>

<style>
#main #center #theme {
    line-height: 1.7rem;
}
#main #center #content p{ 
    padding:0.2rem 0;
} 
#main #container {
    margin-bottom: 1rem;
}
#main a {
    color: #0541af;
}
#main #center #theme #title #save {
    background-color:#6ceff3;
    font-size: 14px;
    padding: 0.1rem 0.2rem;
    border: 1px solid rgb(223, 223, 223);
}
#main #center #theme #title #saved {
    background-color:#15f070;
    font-size: 14px;
    padding: 0.1rem 0.2rem;
    border: 1px solid rgb(223, 223, 223);
}
#main #center #theme, #main #center #comment, #main #center #reply {
    border: 1px solid #ecf0f0;
}
#main #center #theme #title {
    padding: 0.6rem 0.5rem 0.3rem;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #theme #title #theme-title { 
    font-size: 1.4rem;
    font-weight: bold;
}
#main #center #theme #title #info {
    display: inline-block;
    font-size: 14px;
}
#main #center #theme #content {
    padding: 0.5rem;
}
#main hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#main #center #comment #count {
    font-weight: bold;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #comment #detail {
    display: flex;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#main #center #comment #detail #comment_item {
    flex: 1;
    padding: 0.2rem;
}
#main #center #comment #content {
    line-height: 1.5rem;
    font-size: 0.9rem;
}
#main #center #comment #detail #info{
    font-size: 14px;
}
#main #center #reply {
    border-top: 1px solid #7B463D;
}
#main #center #content h1,
#main #center #content h2,
#main #center #content h3,
#main #center #content h4,
#main #center #content h5,
#main #center #content h6 {
    padding: 1.1vh 0;
    line-height: 2.2rem;
}
#main #center #editor {
    margin: auto;
}
#main #center iframe {
    margin: 0.5rem auto;
    width: 99%;
}
#main #center img {
    margin: 1rem auto;
    padding: 0.1rem;
    width: 100%;
}
#main #center blockquote {
    padding: 0.5rem;
    border-left: 4px solid #ccc;
    background: ghostwhite;
    width: 100%;
}
#main #center ul li {
    margin: auto 1.5rem;
    list-style-type:square;
}
#main #center ol li {
    margin: auto 1.6rem;
    list-style-type:decimal;
}
#main #center table {
    padding: 0.3rem;
    border: 2px solid  #aaa;
}
#main #center table td {
    padding: 0.5rem;
    border-top: 1px solid  #aaa;
}
#main #center pre {
    display: block;
    padding: 5px;
    margin: 5px 0;
    line-height: 1.5rem;
    word-break: break-all;
    word-wrap: break-word;
    background-color: #f0f0ff;
    border: 1px solid #e5e6f8;
    text-shadow: none;
    overflow-x: auto;
}
#main #center code {
    padding: 2px 4px;
    background-color: #f5f5f5;
    border-radius: 4px;
    border: 1px solid #ccc;
    color: var(--purple);
    text-shadow: none;
}
#main #center pre code {
    padding: 0;
    font-size: 0.9rem;
    color: inherit;
    /* white-space: pre-wrap; */
    background-color: transparent;
    border-radius: 0;
    border: 0;
}
@media only screen and (max-width: 767px) {
    #main{
        margin: 0 auto;
        width: 97%;
        padding-top: 3.5rem;
    }
    #container #center {
        margin-bottom: 1rem;
    }
    #main #container #center #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin: 0.2rem 0.3rem 0;
        border-radius: 50%;
    }
    #main #center iframe {
        height: 444px;
    }
}
@media only screen and (max-width: 500px) {
    #main #center iframe {
        height: 300px;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    #main  {
        margin: 0 auto;
        padding-top: 4rem;
    }
    #container {
        margin: 0 auto;
        width: 77%;
    }
    #container #center {
        margin-bottom: 1rem;
    }
    #main #container #center #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin: 0.2rem 0.3rem 0;
        border-radius: 50%;
    }
    #main #center iframe {
        height: 466px;
    }
}
@media only screen and (min-width: 1000px) {
    #main {
        margin: 0 auto;
        padding-top: 4rem;
    }
    #main #container {
        margin: 0 auto;
        width: 1000px;
        display: flex;
        flex-flow: row;
    }
    #main #container #center {
        width: 75%;
        margin-right: 0.8rem;
    }
    #main #container #center #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin: 0.2rem 0.3rem 0;
        border-radius: 50%;
    }
    #main #container #aside {
        flex: 1;
    }
    #main #center iframe {
        height: 466px;
    }
}
</style>