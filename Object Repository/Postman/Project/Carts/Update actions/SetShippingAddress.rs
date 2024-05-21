<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetShippingAddress</name>
   <tag></tag>
   <elementGuidId>0cd6fa5d-a0ce-454e-9358-cc93475bf07b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>ABWVE3YeqC03GskNkZr8l-z9n8K0F3hG</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${GlobalVariable.cart_version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setShippingAddress\&quot;,\n            \&quot;address\&quot; : {\n              \&quot;key\&quot; : \&quot;ExampleKey\&quot;,\n              \&quot;title\&quot; : \&quot;My Address\&quot;,\n              \&quot;salutation\&quot; : \&quot;Mr.\&quot;,\n              \&quot;firstName\&quot; : \&quot;Anjali\&quot;,\n              \&quot;lastName\&quot; : \&quot;K\&quot;,\n              \&quot;streetName\&quot; : \&quot;Example Street\&quot;,\n              \&quot;streetNumber\&quot; : \&quot;4711\&quot;,\n              \&quot;additionalStreetInfo\&quot; : \&quot;Backhouse\&quot;,\n              \&quot;postalCode\&quot; : \&quot;80933\&quot;,\n              \&quot;city\&quot; : \&quot;Exemplary City\&quot;,\n              \&quot;region\&quot; : \&quot;Exemplary Region\&quot;,\n              \&quot;country\&quot; : \&quot;DE\&quot;    \n             \n            }\n          }\n    ]\n}\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4b35a346-b9ad-4e78-8596-91b7d97d0370</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.ctp_access_token}</value>
      <webElementGuid>e38b32c2-f68a-4d67-8494-b605be5c5917</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/${GlobalVariable.project_key}/carts/${GlobalVariable.cart_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>94abcb84-fc9a-4d92-9850-1f84abcd4245</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project_key</defaultValue>
      <description></description>
      <id>013d4362-7533-4c44-beeb-a973e68f6b4e</id>
      <masked>false</masked>
      <name>project_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_id</defaultValue>
      <description></description>
      <id>39d24959-3c2f-4994-a4c9-92905cc492a5</id>
      <masked>false</masked>
      <name>cart_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_version</defaultValue>
      <description></description>
      <id>72648084-3fb6-468c-8785-d894888bc940</id>
      <masked>false</masked>
      <name>cart_version</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'id', &quot;5e605f86-1fd4-41ee-ba39-4fa4b673adaa&quot;)
WS.verifyElementPropertyValue(response, 'shippingAddress', '')
WS.verifyElementPropertyValue(response, 'shippingAddress.streetName', &quot;Anjali&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
