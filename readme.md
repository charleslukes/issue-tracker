## ISSUE TRACKER

#### GOALS
- Read file
- Search through file
- Find text with `TODO` and `DESCRIPTION`
- Make API call to github and create an issue with the text
- Check if issue is already created 

#### USAGE
On your terminal run 
```bash
   AUTH_KEY=xxxxx cargo run -- test.js
```
- Where 
    - xxxxx is your github auth key see how you get it [here](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
    - test.js is the file you want to read


#### Future Improvements 
- Read all files in a folder 
