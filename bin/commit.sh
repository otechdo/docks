#!/bin/bash

TEXTDOMAIN="commit"
TEXTDOMAINDIR="/usr/share/locale/"
. gettext.sh

AUTHOR=$(git config --global --get user.name)
EMAIL=$(git config --global --get user.email)
DATE=$(date +"%Y-%m-%d")
NEW_FEATURE_OR_ENHANCEMENT="New feature or enhancement"
BUG_FIX_OR_ERROR_RESOLUTION="Bug fix or error resolution"
CODE_REFACTORING="Code refactoring"
PERFORMANCE_IMPROVEMENT="Performance improvement"
DOCUMENTATION_OR_CLARITY_IMPROVEMENT="Documentation or clarity improvement"
CODE_CLEANUP_AND_MAINTENANCE="Code cleanup and maintenance"
TESTING_RELATED_CHANGES="Testing-related changes"
MINOR_UPDATES_OR_FIXES="Minor updates or fixes"
INFRASTRUCTURE_CHANGES="Infrastructure changes"
REMOVING_LARGE_CHUNKS_OF_CODE_OR_FEATURES="Removing large chunks of code or features"
MERGING_BRANCHES_OR_CONNECTING_CODE_PARTS="Merging branches or connecting code parts"
INITIAL_COMMIT_OR_MAJOR_FEATURE_START="Initial commit or major feature start"
DEPLOYING_TO_PRODUCTION_OR_RELEASING_A_VERSION="Deploying to production or releasing a version"
SIGNIFICANT_PERFORMANCE_IMPROVEMENTS="Significant performance improvements"
PROJECT_MANAGEMENT_CHANGES="Project management changes"
URGENT_HOTFIXES="Urgent hotfixes"
MAJOR_MILESTONE_OR_GOAL_COMPLETION="Major milestone or goal completion"
INITIAL_INTEGRATIONS_WITH_EXTERNAL_SYSTEMS="Initial integrations with external systems"
IMPROVING_DOCUMENTATION_OR_COMMUNICATION="Improving documentation or communication"
TEMPORARILY_MASKING_FUNCTIONALITY="Temporarily masking functionality"
MAJOR_TRANSFORMATIVE_CHANGE="Major, transformative change"
SERIES_OF_SMALL_CHANGES_OR_FIXES="Series of small changes or fixes"
REFACTORING_CODE_STRUCTURE="Refactoring code structure"
TEMPORARILY_DISABLING_A_FEATURE="Temporarily disabling a feature"
INITIAL_IMPLEMENTATION_OF_A_FEATURE="Initial implementation of a feature"
RAPID_IMPACTFUL_CHANGES="Rapid, impactful changes"
MINOR_TEMPORARY_CHANGE="Minor, temporary change"
BRINGS_THE_PROJECT_CLOSER_TO_ITS_GOALS_OR_OBJECTIVES="Brings the project closer to its goals or objectives"
IMMEDIATE_GOALS_BUT_IS_NECESSARY_FOR_LONG_TERM_PROGRESS="Immediate goals, but is necessary for long-term progress"
IMPROVING_CODE_COMMENTS_OR_DOCUMENTATION="Improving code comments or documentation"
EXPANDING_A_FEATURE_OR_FUNCTIONALITY="Expanding a feature or functionality"
OPTIMIZING_CODE_FOR_PERFORMANCE="Optimizing code for performance"
MERGING_FEATURES_OR_COMPONENTS="Merging features or components"
UNDEVELOPED_FEATURE_WITH_POTENTIAL="Undeveloped feature with potential"
EXPERIMENTAL_OR_SPECULATIVE_CHANGE="Experimental or speculative change"
INDEPENDENT_CHANGE="Independent change"
CREATION_OF_NEW_COMPONENTS="Creation of new components"
REMOVAL_OR_DEPRECATION_OF_A_COMPONENT="Removal or deprecation of a component"
COLLECTION_OF_RELATED_CHANGES="Collection of related changes"
REMOVAL_OF_A_MODULE_COMPONENT_OR_FEATURE="Removal of a module, component, or feature"
RESOLVING_MERGE_CONFLICTS_OR_DEPENDENCIES="Resolving merge conflicts or dependencies"
MIXING_UNKNOWN_OR_MYSTERIOUS_BUGS="Mixing unknown or mysterious bugs"
IMPROVING_CODE_PERFORMANCE="Improving code performance"
CHANGES_TO_DATE_TIME_OR_SCHEDULING="Changes to date, time, or scheduling"
ALTERING_DATA_OR_INFORMATION_FLOW="Altering data or information flow"
CONNECTING_CODE_PARTS="Connecting code parts"
SMALL_RANDOM_CHANGE="Small, random change"
REMOVING_TECHNICAL_DEBT="Removing technical debt"
ESTABLISHING_CLOSE_RELATIONSHIPS_BETWEEN_CODE_PARTS="Establishing close relationships between code parts"
SLOWING_DOWN_OR_REDUCING_CODE_PERFORMANCE="Slowing down or reducing code performance"
TESTING_NEW_FEATURES_OR_TECHNOLOGIES="Testing new features or technologies"
CREATING_OR_IMPROVING_ENVIRONMENTS="Creating or improving environments"
DEPLOYING_TO_PRODUCTION="Deploying to production"
URGENT_PRODUCTION_HOTFIXES="Urgent production hotfixes"
MAKING_CODEBASE_MORE_ACCESSIBLE="Making codebase more accessible"
SIGNIFICANT_SPEED_IMPROVEMENT="Significant speed improvement"
COMPREHENSIVE_OPTIMIZATION_OF_A_SPECIFIC_AREA="Comprehensive optimization of a specific area"
LONG_TERM_PROJECT_FOR_A_SELF_SUSTAINING_SYSTEM="Long-term project for a self sustaining system"
STABILIZING_OR_BALANCING_CODE_PARTS="Stabilizing or balancing code parts"
CHANGING_PROJECT_DIRECTION="Changing project direction"
REPRESENTS_PROJECT_MANAGEMENT_RELATED_CHANGES="Represents project management-related changes"
CELEBRATES_THE_COMPLETION_OF_MAJOR_MILESTONES="Celebrates the completion of major milestones"
MIGRATION_TO_A_NEW_ARCHITECTURE_OR_LANGUAGE="Migration to a new architecture or language"
EXPLORATION_OF_NEW_TECHNOLOGIES_OR_APPROACHES="Exploration of new technologies or approaches"
RESOLUTION_OF_A_COMPLEX_OR_HARD_TO_REPRODUCE_ISSUE="Resolution of a complex or hard-to-reproduce issue"
CHANGES_RELATED_TO_TIME_DATES_OR_TIMESTAMPS="Changes related to time, dates, or timestamps"
SCALING_UP_THE_SYSTEM_OR_INCREASING_CAPACITY=_"Scaling up the system or increasing capacity"
REDUCTION_OF_CODEBASE_SIZE_OR_REMOVAL_OF_FEATURES=_"Reduction of codebase size or removal of features"
ENTER_COMMIT_SCOPE=_"enter the commit scope"

COMMIT_TEMPLATE="%type%(%scope%): %summary%

    The following changes were made :

        %why%

    Authored by :

        * %author% <%email%> the %date%
"

SCOPE=""
SUMMARY=""
WHY=""
function ask() {
  clear
  while [ -z "${SCOPE}" ]
  do
        printf "\033[1;34m%s :\033[1;35m " "${ENTER_COMMIT_SCOPE}"
        read -r SCOPE
  done

  while [ -z "${SUMMARY}" ]
  do
        printf "\033[1;34m%s :\033[1;35m " "${ENTER_COMMIT_SCOPE}"
        read -r SUMMARY
  done

  while [ -z "${WHY}" ]
  do
      printf "\033[1;37m[\033[1;34m WHY \033[1;37m]\033[1;35m explain the changes\033[0m : "
      read -r WHY
  done
}

function select_commit_type {
    for i in "${!SHORT_COMMIT_TYPES[@]}"; do
      if [ "$i" -lt 9 ]
      then
            printf "\t\033[1;37m[\033[1;34m 0%d \033[1;37m]\033[1;35m %s\033[0m\n" "$((i+1))" "${SHORT_COMMIT_TYPES[$i]}"
      else
            printf "\t\033[1;37m[\033[1;34m %d \033[1;37m]\033[1;35m %s\033[0m\n" "$((i+1))" "${SHORT_COMMIT_TYPES[$i]}"
      fi
    done
    read -rp "Choose a type of commit (enter the id) : " choice
    if [[ ! $choice =~ ^[0-9]+$ ]] || [[ $choice -lt 1 ]] || [[ $choice -gt ${#SHORT_COMMIT_TYPES[@]} ]]; then
        select_commit_type
    fi
}

function get_scope() {
    read -rp "scope : " SCOPE
}
function get_summary() {
    read -rp "summary : " SUMMARY
}
function get_why() {
    read -rp "why " WHY
}


function main {
    ask
    commit_message=${COMMIT_TEMPLATE/"%type%"/"${COMMIT_TYPES[$choice-1]}"}
    commit_message=${commit_message/"%scope%"/"${SCOPE}"}
    commit_message=${commit_message/"%summary%"/"${SUMMARY}"}
    commit_message=${commit_message/"%why%"/"${WHY}"}
    commit_message=${commit_message/"%author%"/"${AUTHOR}"}
    commit_message=${commit_message/"%email%"/"${EMAIL}"}
    commit_message=${commit_message/"%date%"/"${DATE}"}

    echo "$commit_message"
}

main