# 无限暖暖国服/国际服切换器  
## 使用方式  
### 1.下载游戏  
下载游戏 找到安装路径  
### 2.解压  
将该项目Release解压到无限暖暖游戏本体目录中(与Engine,X6Game等文件夹同级)    
此时文件夹内应该有    
- InfinityNikki/InfinityNikkiGlobal
  - /Engine    
    - /X6Game   
    - /Nikki5-Server-Selector  
        - nikki5-server-selector.exe  
          - cn
              - Launcher
                - launcher.exe
                - 其他文件
          - global  
              - Launcher
                - launcher.exe
                - 其他文件
  - /其他文件  

### 3.运行
运行/Nikki5-Server-Selector/nikki5-server-selector.exe    
选择输入 1 / 2 来确定国服/国际服  
### 4.启动器内选择游戏路径
如果启动器没有识别到该游戏的话    
请打开启动器游戏路径    
将游戏路径设置为步骤1下载的路径  


# 0.0.2版本更新内容  
- 1.新增独立国际服/国服切换器 可做到和普通启动器类似的无感启动    
- 2.新增config.toml配置文件 可在其中修改无限暖暖版本来应对游戏本体的更新  
- 3.游戏版本号由最初的339更新到341 后续若更新可以通过config.toml来切换 
## 注意内容
在游戏本体发布更新之后 若国服和国际服都没有进行下载更新操作  
请勿直接使用此启动器打开  
在使用普通启动器(国服/国际服均可)   
正常打开并更新之后 打开游戏本体文件夹 找到product.db文件中的 version 值   
将其复制到config.toml的game_version = xxx 中(如game_version = 341，注意不需要单引号或双引号)   
复制保存成功之后再次打开切换器即可正常运行     