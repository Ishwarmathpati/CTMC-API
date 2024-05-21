import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput

WebUI.callTestCase(findTestCase('CT/TC2_Creatcart'), [:], FailureHandling.STOP_ON_FAILURE)


//Discount-Fixed price  on the carttotal
CD_FixedResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddDiscountCode', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

def responseText = CD_FixedResponse.getResponseText()

//Double DecimalValue = WS.getElementPropertyValue(Response, JsonPath.toString())
//println(DecimalValue)
println(JsonOutput.println(responseText))

//After discount carttotal price
CarttotalPrice = WS.getElementPropertyValue(CD_FixedResponse, 'totalPrice.centAmount')

println('Fixed price' + '' + CarttotalPrice)

discountamount = WS.getElementPropertyValue(CD_FixedResponse, 'discountOnTotalPrice.discountedAmount.centAmount')

println(('Discounted amount' + '') + discountamount)

if (CarttotalPrice == (GlobalVariable.Cart_total-discountamount)) {
	println('Discount got applied to cart total')
} else {
	println('discount didnt get applied')
}
