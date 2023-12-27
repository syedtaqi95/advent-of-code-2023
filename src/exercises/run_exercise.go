package exercises

import (
	"errors"
)

func RunExercise(day int) error {
	switch day {
	case 2:
		RunDay2()
	default:
		return errors.New("could not find exercise file. Day must be between 2 and 25")
	}
	return nil
}
