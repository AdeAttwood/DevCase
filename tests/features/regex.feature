Feature: Regex search and replace

  Scenario: You can search and replace with with a regular expression
    Given Search is '(\w+)'
    And Replace is 'new'
    And Input is 'This is a'
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
