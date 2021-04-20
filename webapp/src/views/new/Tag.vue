<template>
    <div id="create">
        <main>
            <div id="container">
                <div id="center">
                    <div id="content">
                        <div id="top"><p>新标签(中英名可相同)</p></div><br>
                        <div id="topic">
                                <span  id="category">
                                        <select name="community_name" v-model="CommunityName" @change="choose">
                                            <option v-bind:value="community_name" v-for="(community_name, index) in community_name_tags" :key="index">
                                                    {{community_name.name_cn}}
                                            </option>
                                        </select>
                                </span>
                        </div><br>
                        <input type="text" name="community_tag_name" v-model="CommunityTagName" placeholder="标签名">&emsp;
                        <input type="text" name="community_tag_name_cn" v-model="CommunityTagNameCn" placeholder="标签中文名"><br><br>
                        <button type="submit" id="submit" @click="tag" ><span class="tip"> 新建 </span></button>
                    </div>
                </div>
                <side></side>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Mnav from '../../components/nav/Mnav.vue'
import Side from '../../components/side/Side.vue'
export default {
    name: 'create',
    components: {
        "mnav": Mnav,
        "side": Side
    },
    data () {
        return {
            CommunityTagName: '',
            CommunityTagNameCn: '',
            CommunityName: '',
            community_name: '',
            community_name_tags: ''
        }
    },
    mounted: function() {
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            if (JSON.parse(localStorage.getItem('signin_user')).username === 'admin') {
                let user_id = JSON.parse(localStorage.getItem('signin_user')).id
                let data = { 
                    user_id: Number.parseInt(user_id)
                }
                fetch(URLprefix + 'api/community_names', {
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                }).then(response => response.json())
                .then(json => {
                        this.community_name_tags = json.community_name_tags
                }).catch((e) => {
                    console.log(e)
                })
            }else{
                this.$router.push("/a/signin")
            }
        }else{
            this.$router.push("/a/signin")
        }
    },
    methods: {
        choose() {
            for (let index = 0; index < this.community_name_tags.length; index++) {
                if (this.community_name_tags[index].name_cn === this.CommunityName.name_cn) {
                    this.community_name = this.community_name_tags[index].name
                }
            }
        },
        tag () {
            if (JSON.parse(localStorage.getItem('signin_user'))) {
                let user_id = JSON.parse(localStorage.getItem('signin_user')).id
                let community_name = this.community_name
                let community_tag_name = this.CommunityTagName
                let community_tag_name_cn = this.CommunityTagNameCn
                let data = { 
                    user_id: Number.parseInt(user_id),
                    community_name: community_name,
                    community_tag_name: community_tag_name,
                    community_tag_name_cn: community_tag_name_cn
                }
                console.log(data)
                if(community_name == ''){
                    alert("主题名不能为空, 请选择一个主题")
                    return
                }else if(community_tag_name == ''){
                    alert("标签名不能为空")
                    return
                }else if(community_tag_name_cn == ''){
                    alert("标题中文名不能为空")
                    return
                }else{
                    fetch(URLprefix + 'api/community_tag_new', {
                        body: JSON.stringify(data), 
                        headers: {
                            'content-type': 'application/json'
                        },
                        method: 'POST',
                    }).then(response => response.json())
                    .then(json => {
                            /* window.location.reload ( true ) */
                            this.$router.push("/a/community/"+ community_name)
                    }).catch((e) => {console.log(e)})
                    this.$router.push("/a/community/"+ community_name)
                }
            }else{
                alert("请先登录.")
                this.$router.push("/a/signin")
            }
        }
    }
}
</script>

<style scoped>
#top {
    line-height: 33px;
    border :1px solid #CAC1C1;
}
#topic #category select {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    text-align: center;
    padding-left: 0.5rem;
}
#topic #category select, #topic input {
    height: 30px;
}
#content input {
    line-height: 30px;
}
button {
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
        border: 1px solid #ecf0f0;
    }
    #container #side {
        flex: 1;
    }
}
</style>
