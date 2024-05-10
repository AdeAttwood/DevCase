Feature: Basic search and replace

  Scenario: You can do a basic search and replace of words
    Given Search is 'word'
    And Replace is 'another'
    And Input is 'This is a word'
    Then Output is 'This is a another'

  Scenario: You can do multiple search and replaces
    Given Search is 'word'
    And Replace is 'another'
    And Input is 'This is a word and another word'
    Then Output is 'This is a another and another another'

  Scenario: You can search and replace with multiple words
    Given Search is 'another word'
    And Replace is 'word another'
    And Input is 'This is a word and another word'
    Then Output is 'This is a word and word another'
