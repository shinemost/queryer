## neno crate好像已经不维护了目前，crate.io上只有 0.0.1且不能用

## 此处该用 napi-rs

 - pnpm add -g @napi-rs/cli #安装脚手架工具
 - napi new #新建项目，需要选择构建平台，可全平台编译
 - cd queryer_js && pnpm run build 编译
 - 会生成 index.js、index.d.ts、对应平台的binding文件，此处是 queryer_js.darwin-x64.node
 - 在前端项目中即可直接调用暴露的方法。

## 以下在node环境中执行：
 - const q = require('.')
 - const sql = q.exampleSql()
 - console.log(q.query(sql,'csv'))
 - console.log(q.query(sql,'json'))