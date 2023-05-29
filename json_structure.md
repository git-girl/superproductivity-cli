<!--toc:start-->
- [TOP Level JSON Object Keys](#top-level-json-object-keys)
  - [Metric](#metric)
  - [Obstructions](#obstructions)
  - [Notes](#notes)
  - [Projects](#projects)
  - [ReminderS](#reminders)
  - [simpleCounter](#simplecounter)
  - [tag](#tag)
  - [task](#task)
  - [taskArchive](#taskarchive)
  - [taskRepeatCfg](#taskrepeatcfg)
<!--toc:end-->

This is the top level structure

## TOP Level JSON Object Keys

```json
[
  "bookmark",
  "globalConfig",
  "improvement",
  "lastLocalSyncModelChange",
  "metric",
  "note",
  "obstruction",
  "project",
  "reminders",
  "simpleCounter",
  "tag",
  "task",
  "taskArchive",
  "taskRepeatCfg"
]
```
**What to keep**

Can ditch bookmark, global Config, lastLocalSyncModelChange

lastLocalSyncModelChange  is some number that id quite know about yet
metric is the how did you feel thing

### Metric
`x["metric"]["entities"]` => dates as keys with keys: 
```
  "id": "2023-03-13",
  "improvements": [],
  "improvementsTomorrow": [],
  "mood": 8,
  "obstructions": [],
  "productivity": 6
```
and x metric ids has an array of ids  

### Obstructions

obstructions is practically the same in the entities id thing 
Obstructions: 
```json
id:
title:
```

### Notes

Notes also consists out of entities and ids and a today order
./target/debug/superproductivity-cli | fx 'x["note"]["entities"][x["note"]["ids"].first]'  
```json 
{
  "content": "redacted text is redacted \n",
  "created": 1678700969313,
  "id": "6DvtwMsvWjA0IUZdTg8LU",
  "isPinnedToToday": false,
  "modified": 1678700969313,
  "projectId": "OH88qnI802CTlIci_jz6g"
}
```

### Projects

`./target/debug/superproductivity-cli | fx 'x["project"]["entities"][x["project"]["ids"].first]'`

```json 
{
  "advancedCfg": {
    "worklogExportSettings": {
      "cols": [
        "DATE",
        "START",
        "END",
        "TIME_CLOCK",
        "TITLES_INCLUDING_SUB"
      ],
      "groupBy": "DATE",
      "roundEndTimeTo": null,
      "roundStartTimeTo": null,
      "roundWorkTimeTo": null,
      "separateTasksBy": " | "
    }
  },
  "backlogTaskIds": [
    "4519JB54gmFYB1pxnArud",
    "8lTH6t1HrtMxuPQ6iVKEi"
  ],
  "breakNr": {
    "2022-11-07": 1,
    "2022-11-09": 1,
    "2022-11-10": 2,
    "2022-11-12": 1,
    "2022-11-16": 2,
    "2022-12-05": 1,
    "2023-02-04": 1
  },
  "breakTime": {
    "2022-11-07": 300000,
    "2022-11-09": 300000,
    "2022-11-10": 10449325,
    "2022-11-12": 1102303,
    "2022-11-16": 3671905,
    "2022-12-05": 300000,
    "2023-02-04": 4112917
  },
  "id": "OH88qnI802CTlIci_jz6g",
  "isArchived": false,
  "isHiddenFromMenu": false,
  "issueIntegrationCfgs": {
    "CALDAV": {
      "caldavUrl": null,
      "categoryFilter": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromCaldav": false,
      "isTransitionIssuesEnabled": false,
      "password": null,
      "resourceName": null,
      "username": null
    },
    "GITEA": {
      "host": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromGitea": false,
      "repoFullname": null,
      "scope": "created-by-me",
      "token": null
    },
    "GITHUB": {
      "filterUsername": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromGithub": false,
      "repo": null,
      "token": null
    },
    "GITLAB": {
      "filterUsername": null,
      "gitlabBaseUrl": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromGitlab": false,
      "project": null,
      "scope": "created-by-me",
      "source": "project",
      "token": null
    },
    "JIRA": {
      "_isBlockAccess": false,
      "autoAddBacklogJqlQuery": "assignee = currentUser() AND sprint in openSprints() AND resolution = Unresolved",
      "availableTransitions": [],
      "host": null,
      "isAddWorklogOnSubTaskDone": true,
      "isAllowSelfSignedCertificate": false,
      "isAutoAddToBacklog": true,
      "isAutoPollTickets": true,
      "isCheckToReAssignTicketOnTaskStart": false,
      "isEnabled": false,
      "isShowComponents": true,
      "isTransitionIssuesEnabled": true,
      "isUpdateIssueFromLocal": false,
      "isWonkyCookieMode": false,
      "isWorklogEnabled": true,
      "password": null,
      "searchJqlQuery": "",
      "storyPointFieldId": null,
      "transitionConfig": {
        "DONE": "ALWAYS_ASK",
        "IN_PROGRESS": "ALWAYS_ASK",
        "OPEN": "DO_NOT"
      },
      "usePAT": false,
      "userName": null,
      "userToAssignOnDone": null,
      "worklogDialogDefaultTime": "AllTime"
    },
    "OPEN_PROJECT": {
      "availableTransitions": [],
      "filterUsername": null,
      "host": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromOpenProject": false,
      "isSetProgressOnTaskDone": false,
      "isShowTimeTrackingDialog": false,
      "isShowTimeTrackingDialogForEachSubTask": false,
      "isTransitionIssuesEnabled": false,
      "progressOnDone": 0,
      "projectId": null,
      "scope": "created-by-me",
      "timeTrackingDialogDefaultTime": "AllTime",
      "token": null,
      "transitionConfig": {
        "DONE": "ALWAYS_ASK",
        "IN_PROGRESS": "ALWAYS_ASK",
        "OPEN": "DO_NOT"
      }
    },
    "REDMINE": {
      "api_key": null,
      "host": null,
      "isAutoAddToBacklog": false,
      "isAutoPoll": false,
      "isEnabled": false,
      "isSearchIssuesFromRedmine": false,
      "projectId": null,
      "scope": "assigned-to-me"
    }
  },
  "noteIds": [
    "6DvtwMsvWjA0IUZdTg8LU",
    "-vxB4O8-zSEwJWYAl7J6E",
    "a4Bq4MqnSTg7jyGHbSY2h"
  ],
  "taskIds": [
    "yyiYheHywAlMFFTfWcc39",
    "xLLxbKMdyr6ft80FXO2OC",
    "Xp6i5XpmSPtw9u7bSI7M9",
    "iQr0p_L0rY-q4N-QO5gxK",
    "F1jaxta7QFP40g-EhyAEU",
    "xEEbec0RJ1eGw1CTLuYEg",
    "zmV2XYyijQUrN76dWmONU",
    "9zYzi3BQWey5re5F0CByK",
    "Bn-QMhoBVB6YqVjpLG3sx",
    "SE-qxDBOM-mFbtRNSVeIK",
    "UPrcSOfrEZnMKhIRrYBck",
    "0W-2U9d0SDmXN6F5V9vo-",
    "Cz82jHZEn_YWeVLpUgBQz",
    "UcN50ngfkjNG1JtODA-TQ",
    "pGAVnuuEXQw0J8W0O3LDt",
    "rsimn0pN7Azpv229G420s",
    "AQK_DcpEiwXsggjEO62bR",
    "XQGYTUzW2kGSy85Vdxnoh",
    "E476ONxyTxOSpQtTmCJZH",
    "8_DIsYup5RLO4oqOUtMlw"
  ],
  "theme": {
    "accent": "#ff4081",
    "backgroundImageDark": null,
    "backgroundImageLight": null,
    "hueAccent": "500",
    "huePrimary": "500",
    "hueWarn": "500",
    "isAutoContrast": true,
    "isDisableBackgroundGradient": false,
    "primary": "#29b6f6",
    "warn": "#e11826"
  },
  "title": "Other Stuff",
  "workEnd": {
    "2022-11-05": 1667675900025,
    "2022-11-06": 1667731103465,
    "2022-11-10": 1668109454989,
    "2022-11-13": 1668368797110,
    "2022-11-15": 1668531983708,
    "2022-11-16": 1668592501362,
    "2022-11-17": 1668722681574,
    "2022-11-19": 1668863453918,
    "2022-11-21": 1669043395492,
    "2023-02-04": 1675536954397,
    "2023-02-18": 1676744535200,
    "2023-02-21": 1676980561550,
    "2023-03-01": 1677697155719,
    "2023-03-15": 1678881051702,
    "2023-03-16": 1679006844627
  },
  "workStart": {
    "2022-11-05": 1667644855950,
    "2022-11-06": 1667730940464,
    "2022-11-10": 1668096977100,
    "2022-11-13": 1668361304110,
    "2022-11-15": 1668531977708,
    "2022-11-16": 1668591653362,
    "2022-11-17": 1668722680146,
    "2022-11-19": 1668859673923,
    "2022-11-21": 1669043378498,
    "2023-02-04": 1675517930397,
    "2023-02-18": 1676744533421,
    "2023-02-21": 1676980559605,
    "2023-03-01": 1677676415171,
    "2023-03-15": 1678881051702,
    "2023-03-16": 1679006840629
  }
}
```
I think this one shoudld probably be reduced quite a bit 
but i also practically only need the task and note ids id title and maybe maybe the workEnds and workStarts

### ReminderS

**Different!** its just an array!
**Also key is "reminders" note the plural in the main json Object**

`./target/debug/superproductivity-cli | fx 'x["reminders"][0]'`
```json
{
  "id": "8ybouAk5OpkxpJDp09xw2",
  "relatedId": "ksog-iFZbLXsjksLlP46J",
  "remindAt": 1687507200000,
  "title": "Do some thing",
  "type": "TASK"
}
```

### simpleCounter 

follows entities ids schema

`./target/debug/superproductivity-cli | fx 'x["simpleCounter"]["entities"][ x["simpleCounter"]["ids"].first] '`
```json 
{
  "countOnDay": {},
  "icon": "airline_seat_recline_normal",
  "iconOn": "directions_walk",
  "id": "STANDING_DESK_ID",
  "isEnabled": false,
  "isOn": false,
  "title": "Standing Desk Timer",
  "triggerOffActions": [],
  "triggerOnActions": [],
  "type": "StopWatch"
}
```

### tag 

follows entities ids schema   

first id is "TODAY"  

`./target/debug/superproductivity-cli | fx 'x["tag"]["entities"][ x["tag"]["ids"].last] '    `

```json 
{
  "advancedCfg": {
    "worklogExportSettings": {
      "cols": [
        "DATE",
        "START",
        "END",
        "TIME_CLOCK",
        "TITLES_INCLUDING_SUB"
      ],
      "groupBy": "DATE",
      "roundEndTimeTo": null,
      "roundStartTimeTo": null,
      "roundWorkTimeTo": null,
      "separateTasksBy": " | "
    }
  },
  "breakNr": {
    "2023-03-26": 1
  },
  "breakTime": {
    "2023-03-26": 300000
  },
  "color": null,
  "created": 1676652843021,
  "icon": null,
  "id": "1y-ANYKtwGIS6V6cjVmpn",
  "taskIds": [
    "g-u3rIf6yOo7EIKMQMevY"
  ],
  "theme": {
    "accent": "#ff4081",
    "backgroundImageDark": null,
    "backgroundImageLight": null,
    "hueAccent": "500",
    "huePrimary": "500",
    "hueWarn": "500",
    "isAutoContrast": true,
    "isDisableBackgroundGradient": false,
    "primary": "#a05db1",
    "warn": "#e11826"
  },
  "title": "Calendar",
  "workEnd": {
    "2023-02-21": 1676980561550,
    "2023-02-27": 1677531697256,
    "2023-03-29": 1680093628489,
    "2023-04-17": 1681752529235
  },
  "workStart": {
    "2023-02-21": 1676980559605,
    "2023-02-27": 1677503994154,
    "2023-03-29": 1680091511489,
    "2023-04-17": 1681752528264
  }
}
```

### task 

task has the top level keys: 
```json 
[
  "__modelVersion",
  "currentTaskId",
  "entities",
  "ids",
  "isDataLoaded",
  "lastCurrentTaskId",
  "selectedTaskId",
  "taskAdditionalInfoTargetPanel"
]
```
where currentTaskId, isDataLoaded, lastCurrentTaskId, selectedTaskId and taskAdditionalInfoTargetPanel are less important

task entity: 
```json 
{
  "_showSubTasksMode": 2,
  "attachments": [],
  "created": 1685359639895,
  "doneOn": null,
  "id": "tcQx9liVYXJiFwXkZe_Wq",
  "isDone": false,
  "issueAttachmentNr": null,
  "issueId": null,
  "issueLastUpdated": null,
  "issuePoints": null,
  "issueType": null,
  "issueWasUpdated": null,
  "notes": "",
  "parentId": null,
  "plannedAt": null,
  "projectId": null,
  "reminderId": null,
  "repeatCfgId": null,
  "subTaskIds": [],
  "tagIds": [
    "TODAY"
  ],
  "timeEstimate": 0,
  "timeSpent": 0,
  "timeSpentOnDay": {},
  "title": "test2\u00a0"
}
```
### taskArchive 

this mirrors `task` but contains completed tasks 

top level keys are: 

```json 
[
  "__modelVersion",
  "currentTaskId",
  "entities",
  "ids",
  "isDataLoaded",
  "lastCurrentTaskId",
  "selectedTaskId",
  "taskAdditionalInfoTargetPanel"
]
```

a taskArchive entity looks like this:   
`./target/debug/superproductivity-cli | fx 'x["taskArchive"]["entities"][ x["taskArchive"]["ids"].first ]'`  
```json 
{
  "_showSubTasksMode": 2,
  "attachments": [],
  "created": 1667662496129,
  "doneOn": 1667679513787,
  "id": "qAL98Gzyt21kuflZJhM1U",
  "isDone": true,
  "issueAttachmentNr": null,
  "issueId": null,
  "issueLastUpdated": null,
  "issuePoints": null,
  "issueType": null,
  "issueWasUpdated": null,
  "notes": "",
  "parentId": null,
  "plannedAt": null,
  "projectId": null,
  "reminderId": null,
  "repeatCfgId": null,
  "subTaskIds": [],
  "tagIds": [
    "TODAY"
  ],
  "timeEstimate": 0,
  "timeSpent": 0,
  "timeSpentOnDay": {},
  "title": "things"
}
```

### taskRepeatCfg

follows entities ids schema

`./target/debug/superproductivity-cli | fx 'x["taskRepeatCfg"]["entities"][ x["taskRepeatCfg"]["ids"].first ]'`
```json 
{
  "defaultEstimate": 7200000,
  "friday": true,
  "id": "YuMksAdEBC5HcX8fys7iR",
  "isPaused": false,
  "lastTaskCreation": 1685313422534,
  "monday": true,
  "notes": "Redacted",
  "order": 0,
  "projectId": "xprPZ--0o2vE17fGOA_kT",
  "quickSetting": "CUSTOM",
  "remindAt": "AtStart",
  "repeatCycle": "WEEKLY",
  "repeatEvery": 1,
  "saturday": true,
  "startDate": "2022-11-06",
  "startTime": "8:00",
  "sunday": true,
  "tagIds": [
    "TODAY"
  ],
  "thursday": true,
  "title": "Stuff",
  "tuesday": true,
  "wednesday": true
}
```

