Feature: Regex search and replace

  Scenario: You can search and replace with with a regular expression
    Given Search is '(\w+)'
    And Replace is 'new'
    And Input is 'this is a'
    Then Output is 'new new new'

  Scenario: You can use a '$' to replace a match group
    Given Search is 'function (\w+)\(\)'
    And Replace is 'fun $1()'
    And Input is 'function foo()'
    Then Output is 'fun foo()'

  Scenario: You can will need to wrap the match group when the match is against another word
    Given Search is 'Hello (\w+)'
    And Replace is 'Hello ${1}s'
    And Input is 'Hello world'
    Then Output is 'Hello worlds'

  Scenario: You can search with an invalid regular expression
    Given Search is '(\w+'
    And Replace is 'new'
    And Input is 'this is a'
    Then Output is 'this is a'

  Scenario: You can replace a pattern with a dot in it like a css class name
    Given Search is '.testing'
    And Replace is '.another'
    And Input is '.testing {'
    Then Output is '.another {'

  Scenario: You can replace a pattern that grabs all the text
    Given Search is '.*'
    And Replace is '.another'
    And Input is '.testing {'
    Then Output is '.another'
