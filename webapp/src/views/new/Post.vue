<template>
    <div id="post">
        <main>
            <div id="container">
                <div id="center">
                    <div id="new-title"><p>新主题</p></div>
                            <div id="topic-group">
                                <span  id="category">
                                        <select name="community_name" v-model="CommunityName" @change="choose">
                                            <option v-bind:value="community_name" v-for="(community_name, index) in community_names" :key="index">
                                                    {{community_name.name_cn}}
                                            </option>
                                        </select>
                                </span>&nbsp; 
                                <span  id="tag">
                                        <select v-if="username == 'admin'"  name="community_tag_name" v-model="CommunityTagName">
                                            <option v-bind:value="community_tag_name" v-for="(community_tag_name, index) in community_tag_names_admin" :key="index">
                                                    {{community_tag_name}}
                                            </option>
                                        </select>
                                        <select v-else name="community_tag_name" v-model="CommunityTagName">
                                            <option v-bind:value="community_tag_name" v-for="(community_tag_name, index) in community_tag_names" :key="index">
                                                    {{community_tag_name}}
                                            </option>
                                        </select>
                                </span>
                                <span id="title">
                                        <input type="text" name="title" v-model="Title" placeholder=" 请填写标题">
                                </span>
                            </div>    
                            <div id="editor">
                                <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                            </div>
                            <div id="new">
                                    <button type="submit" id="submit" @click="postnow" >发布</button>
                            </div>
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
    name: 'post',
    components: {
        mavonEditor,
        "side": Side
    },
    data () {
        return {
            username: '',
            community_names: '',
            community_tag_names: '',
            community_tag_names_admin: '',
            community_name: '',
            CommunityName: '',
            CommunityTagName: '',
            indexNum:0,//定义一级菜单的下标
            Title: '',
            Content: '',
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
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            this.username = JSON.parse(localStorage.getItem('signin_user')).username
            let u_id = JSON.parse(localStorage.getItem('signin_user')).id
            let data = { 
                user_id:  Number.parseInt(u_id)
            }
            fetch(URLprefix + 'api/community_names', {
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
            }).then(response => response.json())
            .then(json => {
                    this.community_names = json.community_name_tags
                    console.log(this.community_names)
            })
            .catch((e) => {
                console.log(e)
            })
        }else{
            alert("请先登录.")
        }
    },
    methods: {
        choose() {
            for (let i = 0; i<this.community_names.length; i++) {
                while (this.community_names[i].name == this.CommunityName.name){
                    this.indexNum = i;
                    break
                }
            }
            for (let index = 0; index < this.community_names.length; index++) {
                if (this.community_names[index].name_cn === this.CommunityName.name_cn) {
                    this.community_name = this.community_names[index].name
                }
            }
            this.community_tag_names_admin = this.community_names[this.indexNum].tags
            let  community_tag_names = []
            this.community_tag_names_admin.filter((item) => { if (item != '宣告') community_tag_names.push(item)})
            this.community_tag_names = community_tag_names
        },
        postnow() {
            if (JSON.parse(localStorage.getItem('signin_user'))) {
                let theme_id = Number.parseInt(0)
                let community_name_cn = this.CommunityName.name_cn
                let community_name = this.community_name
                console.log(community_name)
                let community_tag_name = this.CommunityTagName
                let title = this.Title
                let content = this.Content
                let u_id = JSON.parse(localStorage.getItem('signin_user')).id
                let user_id = Number.parseInt(u_id)
                let data = { 
                    theme_id: theme_id,
                    user_id: user_id,
                    community_name: community_name_cn,
                    community_tag_name: community_tag_name,
                    title: title,
                    content: content
                }
                if(community_name == ''){
                    alert("主题名不能为空, 请选择一个主题")
                    return
                }else if(community_tag_name == ''){
                    alert("标签不能为空,请选择一个标签")
                    return
                }else if(title == ''){
                    alert("标题不能为空")
                    return
                }else if(content == ''){
                    alert("内容不能为空")
                    return
                }else{
                    fetch(URLprefix + 'api/theme_new_edit', {
                        method: 'POST',
                        body: JSON.stringify(data), 
                        headers: {
                            'content-type': 'application/json'
                        },
                        mode: 'cors'
                    }).then(response => response.json()).then(json => {
                        this.$router.push('/')
                        // this.$router.push('/a/community/' + community_name)
                    }).catch( e =>  console.log(e) )
                    this.$router.push('/')
                    // this.$router.push('/a/community/' + community_name)
                }
            }else{
                alert("请先登录.")
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
    /* background-color: #fde7fd; */
}
#center  #topic-group {
   margin: 11px 0 11px 0;
}
#center  #topic-group #category select, #center  #topic-group #tag select {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    text-align: center;
    padding-left: 0.5rem;
}
#center  #topic-group #category select, #center  #topic-group #tag select, #center  #topic-group input {
    height: 30px;
}
#center #editor {
    margin: auto;
}
#container {
    margin-bottom: 1rem;
}
#center  #new button {
    margin-top: 0.3rem;
    width:63px; 
    line-height:25px;
    background-color:#FFFFFF;
    border :1px solid #a39c9c;
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
        padding-top: 4.4rem;
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
    #center  #topic-group input {
        width: 70%;
        float: right;
    }
}
</style>
