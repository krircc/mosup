<template>
    <div id="create">
        <main>
            <div id="container">
                <div id="center">
                    <div id="content">
                        <div id="top"><p>新社区(中英名可相同)</p></div><br>
                        <div id="topic">
                                <span  id="category">
                                        <select name="community_category" v-model="CommunityCategory" >
                                            <option v-bind:value="community_category" v-for="(community_category, index) in community_categorys" :key="index">
                                                    {{community_category}}
                                            </option>
                                        </select>
                                </span>&emsp;
                                <span><input type="text" name="community_category_cn" v-model="CommunityCategoryCn" placeholder="社区分类中文名"></span>
                        </div><br>
                        <input type="text" name="community_name" v-model="CommunityName" placeholder="社区名">&emsp;
                        <input type="text" name="community_name_cn" v-model="CommunityNameCn" placeholder="社区中文名"><br><br>
                        <input type="text" name="community_img" v-model="CommunityImg" placeholder="图片路径"><br><br>
                        <button type="submit" id="submit" @click="create" ><span class="tip"> 新建 </span></button>
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
            CommunityCategory: '',
            CommunityCategoryCn: '',
            CommunityName: '',
            CommunityNameCn: '',
            CommunityImg: '',
            community_categorys: ''
        }
    },
    mounted: function() {
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            if (JSON.parse(localStorage.getItem('signin_user')).username === 'admin') {
                fetch(URLprefix + 'api/community_categorys',{
                    method: 'GET',
                }).then(response => response.json())
                .then(json => {
                        this.community_categorys = json.community_categorys
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
        create () {
            if (JSON.parse(localStorage.getItem('signin_user'))) {
                let community_category = this.CommunityCategory
                let community_category_cn = this.CommunityCategoryCn
                let community_name = this.CommunityName
                let community_name_cn = this.CommunityNameCn
                let community_img = this.CommunityImg
                let user_id = JSON.parse(localStorage.getItem('signin_user')).id
                let data = { 
                    user_id: Number.parseInt(user_id),
                    community_name: community_name,
                    community_name_cn: community_name_cn,
                    community_category: community_category,
                    community_category_cn: community_category_cn,
                    community_img: community_img
                }
                fetch(URLprefix + 'api/community_new', {
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
@media only screen and (max-width: 767px) {
    main {
        margin: 0 auto;
        padding-top: 4rem;
        width: 90%;
        margin-bottom: 1rem;
    }
    main #center {
        border: 1px solid #ecf0f0;
        margin-bottom: 1rem;
    }
    #topic #category , #topic input {
        width: 100%;
    }
}
@media only screen and (min-width: 768px) and (max-width: 1000px) {
    main {
        margin: 0 auto;
        padding-top: 4rem;
        width: 70%;
        margin-bottom: 1rem;
    }
    #container #center {
        border: 1px solid #ecf0f0;
        margin-bottom: 1rem;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        padding-top: 4rem;
        margin-bottom: 1rem;
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
