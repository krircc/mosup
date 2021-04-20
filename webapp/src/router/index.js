import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/home/Home.vue'

const routes = [
    { path: '/', name: 'home', component: Home },
    { path: '/a/wiki', name: 'wiki', component: () => import('../views/wiki/Wiki.vue') },
        { path: '/a/wiki/rust', name: 'rust', component: () => import('../views/wiki/dir/Rust.vue') },
        { path: '/a/wiki/actix', name: 'actix', component: () => import('../views/wiki/dir/Actix.vue') },
        { path: '/a/wiki/yew', name: 'yew', component: () => import('../views/wiki/dir/Yew.vue') },
    { path: '/a/interest', name: 'interest', component: () => import('../views/interest/Interest.vue') },
    { path: '/a/explore', name: 'explore', component: () => import('../views/explore/Explore.vue') },
    { path: '/a/:community/theme/edit/:id', name: 'edit', component: () => import('../views/theme/Edit.vue') },
    { path: '/a/:community/theme/:id', name: 'theme', component: () => import('../views/theme/Theme.vue') },
    { path: '/a/post', name: 'post', component: () => import('../views/new/Post.vue') },
    { path: '/a/tag', name: 'tag', component: () => import('../views/new/Tag.vue') },
    { path: '/a/create', name: 'create', component: () => import('../views/new/Create.vue') },
    { path: '/a/signin', name: 'signin', component: () => import('../views/user/Signin.vue') },
    { path: '/a/signup', name: 'signup', component: () => import('../views/user/SignUp.vue') },
    { path: '/a/user/:id', name: 'hourse', component: () => import('../views/user/Hourse.vue') },
    { path: '/a/user/:id/comment', name: 'usercomment', component: () => import('../views/user/Comment.vue') },
    { path: '/a/user/:id/save', name: 'usersave', component: () => import('../views/user/Save.vue') },
    { path: '/a/user/:id/message', name: 'usermessage', component: () => import('../views/user/Message.vue') },
    { path: '/a/community/:name', name: 'community', component: () => import('../views/community/Community.vue') },
    { path: '/a/community/:name/:tag', name: 'community-tag', component: () => import('../views/community/CommunityTag.vue') },
    // { path: '*', name: 'notfound', component: () => import('../views/notfound/NotFound.vue') },
]

const router = createRouter({
history: createWebHistory(),
routes,
})

export default router