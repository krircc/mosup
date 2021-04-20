<template>
    <div id="edit">
        <main>
            <div id="container">
                <div id="center">
                    <div id="new-title"><p>主题更新</p></div>
                    <form id="form" >
                            <div id="topic-group">
                                <input type="text" name="title" v-model="Title" placeholder="请输入主题名">
                            </div>    
                            <div id="editor">
                                <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                            </div>
                            <div id="new">
                                    <button type="submit" id="submit" @click="update" >发布</button>
                            </div>
                    </form>
                </div>
               <side></side>
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
    name: 'edit',
    components: {
        mavonEditor,
        "side": Side
    },
    data () {
        return {
            community_tag_name: '',
            Title: '',
            Content: '',
            set:{
                bold: true, // 粗体
                italic: true, // 斜体
                header: true, // 标题
                quote: true, // 引用
                ul: true, // 无序列表
                ol: true, // 有序列表
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
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            let theme_id = Number.parseInt(this.$route.params.id)
            let user_id = JSON.parse(localStorage.getItem('signin_user')).id
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
                this.community_tag_name = json.theme.community_tag_name
                this.Title = json.theme.title
                this.Content = json.theme_raw_content
            }).catch((e) => {
                console.log(e)
            })
        }else{
            alert("请先登录.")
        }
    },
    methods: {
        update() {
            if (JSON.parse(localStorage.getItem('signin_user'))) {
                this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
                let theme_id = Number.parseInt(this.$route.params.id)
                let community_name = this.$route.params.community
                let community_tag_name = this.community_tag_name
                let title = this.Title
                let content = this.Content
                let user_id = JSON.parse(localStorage.getItem('signin_user')).id
                if(title == ''){
                    alert("标题不能为空")
                    return
                }else if(content == ''){
                    alert("内容不能为空")
                    return
                }else{
                    let data = { 
                        theme_id: theme_id,
                        user_id: user_id,
                        community_name: community_name,
                        community_tag_name: community_tag_name,
                        title: title,
                        content: content
                    }
                    fetch(URLprefix + 'api/theme_new_edit', {
                        method: 'POST',
                        body: JSON.stringify(data), 
                        headers: {
                            'content-type': 'application/json'
                        },
                        mode: 'cors'
                    }).then(response => response.json())
                    .then(json => {
                        this.$router.push('/')
                    }).catch( e => {console.log(e)})
                    this.$router.push('/')
                    // this.$router.push('/a/community/${community_name}')
                }
            }else{
                alert("非法用户, 请先登陆再操作")
                return
            }
        }
    }
}
</script>

<style scoped>
#center #new-title {
    width: 100%;
    line-height: 33px;
    border :1px solid #CAC1C1;
}
#center form #topic-group input {
   margin: 10px auto;
   height: 30px;
   width: 100%;
   border :1px solid #CAC1C1;
}
#center #editor {
    margin: auto;
}
#center form #new button {
    margin-top: 0.3rem;
    width:63px; 
    line-height:25px;
    background-color:#FFFFFF;
    border :1px solid #a39c9c;
}
#container {
    margin-bottom: 1rem;
}
@media only screen and (max-width: 767px) {
    main{
        padding-top: 4rem;
        margin: 0 auto;
        width: 97%;
    }
    #center  #topic-group #category select, #center  #topic-group #tag select, #center  #topic-group input {
        width: 100%;
    }
    #center {
        display: block;
        margin: 0.2rem 0 1rem;
        border: 1px solid #e4e3e3;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    main {
        margin: 0 auto;
        padding-top: 4rem;
        width: 70%;
    }
    #container #center {
        border: 1px solid #ecf0f0;
        margin-bottom: 1rem;
    }
    #center  #topic-group input {
        width: 65%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        padding-top: 4rem;
        width: 72%;
    }
    #container {
        margin: 0 auto;
        width: 1000px;
        display: flex;
        flex-flow: row;
    }
    #container #center {
        width: 75%;
        margin-right: 0.8rem;
    }
    #container #side {
        flex: 1;
    }
}
</style>